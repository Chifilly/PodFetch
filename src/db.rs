use std::time::SystemTime;
use chrono::{DateTime, Utc};
use diesel::{insert_into, RunQueryDsl, sql_query};
use feed_rs::model::Entry;
use crate::models::itunes_models::{Podcast, PodcastEpisode};
use crate::models::models::{PodcastWatchedEpisodeModelWithPodcastEpisode, PodcastHistoryItem,
                            PodcastWatchedPostModel};
use crate::service::mapping_service::MappingService;
use diesel::prelude::*;
use crate::config::dbconfig::establish_connection;
use crate::schema::podcast_episodes::dsl::podcast_episodes;
use crate::schema::podcast_history_items::dsl::podcast_history_items;

pub struct DB{
    conn: SqliteConnection,
    mapping_service: MappingService
}

impl DB{
    pub fn new() -> Result<DB, String>{
        let conn = establish_connection();
        Ok(DB{conn, mapping_service: MappingService::new()})
    }

    pub fn get_podcasts(&mut self) -> Result<Vec<Podcast>, String>{
        use crate::schema::podcasts::dsl::podcasts;
        let result = podcasts
            .load::<Podcast>(&mut self.conn)
            .expect("Error loading podcasts");
        Ok(result)
    }

    pub fn get_podcast(&mut self, podcast_id_to_be_found: i32) -> Result<Podcast, String>{
        use crate::schema::podcasts::{id as podcast_id};
        use crate::schema::podcasts::dsl::podcasts;
        let found_podcast = podcasts
            .filter(podcast_id.eq(podcast_id_to_be_found))
            .first::<Podcast>(&mut self.conn)
            .expect("Error loading podcast by id");

        Ok(found_podcast)
    }

    pub fn get_podcast_episode_by_id(&mut self, podcas_episode_id_to_be_found: &str) ->
                                                                   Result<Option<PodcastEpisode>, String>{
        use crate::schema::podcast_episodes::{episode_id};
        use crate::schema::podcast_episodes::dsl::*;

        let found_podcast_episode = podcast_episodes
            .filter(episode_id.eq(podcas_episode_id_to_be_found))
            .first::<PodcastEpisode>(&mut self.conn)
            .optional()
            .expect("Error loading podcast by id");

        Ok(found_podcast_episode)
    }


    pub fn get_podcast_episode_by_track_id(&mut self, podcast_id: i32) ->
                                                                   Result<Option<Podcast>, String>{
        use crate::schema::podcasts::{directory};
        use crate::schema::podcasts::dsl::podcasts;
        let optional_podcast = podcasts
            .filter(directory.eq(podcast_id.to_string()))
            .first::<Podcast>(&mut self.conn)
            .optional()
            .expect("Error loading podcast by id");

        Ok(optional_podcast)
    }

    pub fn insert_podcast_episodes(&mut self, podcast: Podcast, link: &str, item: &Entry,
                                   image_url_1: &str, episode_description: &str){
        use crate::schema::podcast_episodes::dsl::*;

        insert_into(podcast_episodes)
            .values((
                podcast_id.eq(podcast.id),
                episode_id.eq(&item.id),
                name.eq(item.title.as_ref().unwrap().clone().content),
                url.eq(link.to_string()),
                date_of_recording.eq(&item.published.unwrap().to_rfc3339()),
                image_url.eq(image_url_1.to_string()),
                description.eq(episode_description)
            ))
            .execute(&mut self.conn)
            .expect("Error inserting podcast episode");
    }

    pub fn add_podcast_to_database(&mut self, collection_name:String, collection_id:String,
                                   feed_url:String, image_url_1: String){
        use crate::schema::podcasts::{directory, rssfeed, name as podcast_name, image_url};
        use crate::schema::podcasts;

        insert_into(podcasts::table)
            .values((
                directory.eq(collection_id.to_string()),
                podcast_name.eq(collection_name.to_string()),
                rssfeed.eq(feed_url.to_string()),
                image_url.eq(image_url_1.to_string())
            ))
            .execute(&mut self.conn)
            .expect("Error inserting podcast");
    }

    pub fn get_last_5_podcast_episodes(&mut self, podcast_episode_id: i32) ->
                                                                      Result<Vec<PodcastEpisode>,
                                                                          String>{
        use crate::schema::podcast_episodes::{date_of_recording, podcast_id};
        let podcasts = podcast_episodes
            .filter(podcast_id.eq(podcast_episode_id))
            .limit(5)
            .order(date_of_recording.desc())
            .load::<PodcastEpisode>(&mut self.conn)
            .expect("Error loading podcasts");
        println!("Podcasts found: {}", podcasts.len());
        Ok(podcasts)
    }


    pub fn get_podcast_episodes_of_podcast(&mut self, podcast_id_to_be_searched: i32, last_id:
    Option<String>) ->
                                                                      Result<Vec<PodcastEpisode>, String>{
        use crate::schema::podcast_episodes::*;
        match last_id {
            Some(last_id) => {
                let podcasts_found = podcast_episodes.filter(podcast_id.eq(podcast_id_to_be_searched))
                    .filter(date_of_recording.lt(last_id))
                    .order(date_of_recording.desc())
                    .limit(75)
                    .load::<PodcastEpisode>(&mut self.conn)
                    .expect("Error loading podcasts");
                Ok(podcasts_found)
            }
            None => {
                let podcasts_found = podcast_episodes.filter(podcast_id.eq(podcast_id_to_be_searched))
                    .order(date_of_recording.desc())
                    .limit(75)
                    .load::<PodcastEpisode>(&mut self.conn)
                    .expect("Error loading podcasts");

                Ok(podcasts_found)
            }
        }


    }

    pub fn log_watchtime(&mut self, watch_model: PodcastWatchedPostModel) ->Result<(), String> {
        let result = self.get_podcast_episode_by_id(&watch_model.podcast_episode_id).unwrap();

        use crate::schema::podcast_history_items;
        match result {
            Some(result)=>{
                let now = SystemTime::now();
                let now: DateTime<Utc> = now.into();
                let now: &str = &now.to_rfc3339();
                insert_into(podcast_history_items)
                    .values((
                        podcast_history_items::podcast_id.eq(result.podcast_id),
                        podcast_history_items::episode_id.eq(result.episode_id),
                        podcast_history_items::watched_time.eq(watch_model.time),
                        podcast_history_items::date.eq(&now),
                    ))
                    .execute(&mut self.conn)
                    .expect("Error inserting podcast episode");
                Ok(())
            }
            None=>{
                panic!("Podcast episode not found");
            }
        }
    }

    pub fn get_watchtime(&mut self, podcast_id: &str) ->Result<PodcastHistoryItem, String>{
        let result = self.get_podcast_episode_by_id(podcast_id).unwrap();
        use crate::schema::podcast_history_items;

        match result {
            Some(found_podcast)=>{

                let history_item = podcast_history_items
                    .filter(podcast_history_items::episode_id.eq(podcast_id))
                    .first::<PodcastHistoryItem>(&mut self.conn)
                    .optional()
                    .expect("Error loading podcast episode by id");
                return match history_item {
                    Some(found_history_item) => {
                        Ok(found_history_item)
                    }
                    None => {
                        Ok(PodcastHistoryItem {
                            id: 0,
                            podcast_id: found_podcast.podcast_id,
                            episode_id: found_podcast.episode_id,
                            watched_time: 0,
                            date: "".to_string(),
                        })
                    }
                }
            }
            None=>{
                panic!("Podcast episode not found");
            }
        }
    }


    pub fn get_last_watched_podcasts(&mut self)
        -> Result<Vec<PodcastWatchedEpisodeModelWithPodcastEpisode>, String> {

        let result = sql_query("SELECT * FROM (SELECT * FROM podcast_history_items ORDER BY \
        datetime\
        (date) \
        DESC) GROUP BY episode_id  LIMIT 10;")
            .load::<PodcastHistoryItem>(&mut self.conn)
            .unwrap();

        let podcast_watch_episode = result.iter().map(|podcast_watch_model|{
            let optional_podcast = self.get_podcast_episode_by_id(&podcast_watch_model.episode_id)
                .unwrap();
        match optional_podcast {
            Some(podcast_episode) => {

                let podcast_dto = self.mapping_service.map_podcastepisode_to_dto(&podcast_episode);
                let podcast = self.get_podcast(podcast_episode.podcast_id).unwrap();
                let podcast_watch_model = self.mapping_service
                    .map_podcast_history_item_to_with_podcast_episode(&podcast_watch_model.clone(),
                                                                      podcast_dto, podcast);
                return podcast_watch_model
            }
            None => {
                panic!("Podcast episode not found");
            }
        }

    }).collect::<Vec<PodcastWatchedEpisodeModelWithPodcastEpisode>>();
        Ok(podcast_watch_episode)
    }

    pub fn update_total_podcast_time_and_image(&mut self, episode_id: &str, time: i32, image_url:
    &str, url: &str ) -> Result<(), String> {
        use crate::schema::podcast_episodes::dsl::episode_id as episode_id_column;
        use crate::schema::podcast_episodes::dsl::total_time as total_time_column;
        use crate::schema::podcast_episodes::dsl::local_image_url as local_image_url_column;
        use crate::schema::podcast_episodes::dsl::local_url as local_url_column;

        let result = podcast_episodes
            .filter(episode_id_column.eq(episode_id))
            .first::<PodcastEpisode>(&mut self.conn)
            .optional()
            .expect("Error loading podcast episode by id");

        match result {
            Some(found_podcast)=>{
                println!("Found podcast: {:?}", found_podcast);
                let new_time = found_podcast.total_time + time;
                diesel::update(podcast_episodes)
                    .filter(episode_id_column.eq(episode_id))
                    .set((
                        total_time_column.eq(new_time),
                        local_image_url_column.eq(image_url),
                        local_url_column.eq(url)
                    ))
                    .execute(&mut self.conn)
                    .expect("Error updating local image url");
                Ok(())
            }
            None=>{
                panic!("Podcast episode not found");
            }
        }
    }

    pub fn update_podcast_image(mut self, id: &str, image_url: &str)
        -> Result<(), String> {
        use crate::schema::podcasts::dsl::image_url as image_url_column;
        use crate::schema::podcasts::dsl::directory;
        use crate::schema::podcasts::dsl::podcasts as dsl_podcast;

        let result = dsl_podcast
            .filter(directory.eq(id))
            .first::<Podcast>(&mut self.conn)
            .optional()
            .expect("Error loading podcast episode by id");
        match result {
            Some(..)=>{
                diesel::update(dsl_podcast.filter(directory.eq(id)))
                    .set(image_url_column.eq(image_url))
                    .execute(&mut self.conn)
                    .expect("Error updating podcast episode");
                Ok(())
            }
            None=>{
                panic!("Podcast episode not found");
            }
        }
    }

    pub fn get_podcast_by_directory(mut self, podcast_id: &str)
        ->Result<Option<Podcast>, String>{
        use crate::schema::podcasts::dsl::directory;
        use crate::schema::podcasts::dsl::podcasts as dsl_podcast;
        let result = dsl_podcast
            .filter(directory.eq(podcast_id))
            .first::<Podcast>(&mut self.conn)
            .optional()
            .expect("Error loading podcast episode by id");
        Ok(result)

    }
}