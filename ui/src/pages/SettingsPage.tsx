import {useTranslation} from "react-i18next";
import {Switcher} from "../components/Switcher";
import {useEffect, useState} from "react";
import {apiURL} from "../utils/Utilities";
import axios, {AxiosResponse} from "axios";
import {Setting} from "../models/Setting";
import {useSnackbar} from "notistack";
import {Loading} from "../components/Loading";
import {ConfirmModal} from "../components/ConfirmModal";
import {PodcastDelete} from "../components/PodcastDelete";
import {OPMLExport} from "../components/OPMLExport";

export const SettingsPage = () => {
    const {t} = useTranslation()
    const [settings, setSettings] = useState<Setting>()
    const {enqueueSnackbar} = useSnackbar()


    useEffect(()=>{
        axios.get(apiURL+"/settings").then((res:AxiosResponse<Setting>)=>{
            setSettings(res.data)
        })
    },[])

    if(settings===undefined){
        return <Loading/>
    }



    return (
        <div className="p-6">
            <ConfirmModal/>
            <h1 className="text-2xl text-center font-bold">{t('settings')}</h1>
            <div className="grid gap-5">
            <div className="bg-slate-900 rounded p-5 text-white">
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-5">
                    <div className="">{t('auto-cleanup')}</div>
                    <div><Switcher checked={settings.autoCleanup} setChecked={()=>{
                        setSettings({...settings, autoCleanup: !settings?.autoCleanup})
                    }}/></div>

                    <span className=" pt-2 pb-2">{t('days-to-keep')}</span>
                    <div><input type="number" className="bg-gray-600 p-2 rounded" value={settings.autoCleanupDays} onChange={(e)=>{
                        setSettings({...settings, autoCleanupDays: parseInt(e.target.value)})
                    }}/></div>
                    <div className="">
                        {t('auto-update')}
                    </div>

                    <div>
                        <Switcher checked={settings.autoUpdate} setChecked={()=>{
                            setSettings({...settings, autoUpdate: !settings?.autoUpdate})
                        }}/>
                    </div>
                    <div className="">
                        {t('number-of-podcasts-to-download')}
                    </div>
                    <div><input type="number" className="bg-gray-600 p-2 rounded" value={settings.podcastPrefill} onChange={(e)=>{
                        setSettings({...settings, podcastPrefill: parseInt(e.target.value)})
                    }}/></div>
                    <div>
                        {t('auto-download')}
                    </div>
                    <div className="mb-4">
                        <Switcher checked={settings.autoDownload} setChecked={()=>{
                            setSettings({...settings, autoDownload: !settings?.autoDownload})
                        }}/>
                        <button className="bg-blue-600 rounded p-2 hover:bg-blue-500 ml-5" onClick={()=>{
                            axios.put(apiURL+"/settings/runcleanup")
                        }}>{t('run-cleanup')}</button>
                    </div>
                </div>
                <div className="flex">
                    <div className="flex-1"></div>
                    <button className="p-2 bg-blue-600 rounded hover:bg-blue-500" onClick={()=>{
                        axios.put(apiURL+"/settings", settings)
                            .then(()=>{
                                enqueueSnackbar(t('settings-saved'), {variant: "success"})
                            })
                    }}>
                        {t('save')}
                    </button>
                </div>
            </div>


                <OPMLExport/>
            <PodcastDelete/>
        </div>
        </div>
    )
}
