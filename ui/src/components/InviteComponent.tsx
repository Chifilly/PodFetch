import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import axios from "axios";
import {apiURL, formatTime} from "../utils/Utilities";
import {Loading} from "./Loading";
import {SubmitHandler, useForm} from "react-hook-form";
import {LoginData} from "./LoginComponent";
import {useTranslation} from "react-i18next";
import {enqueueSnackbar} from "notistack";


export const InviteComponent = ()=>{
        const params = useParams()
        const [invite, setInvite] = useState<Invite>()
        const [errored, setErrored] = useState<boolean>(false)
        const {register, handleSubmit, formState: {}} = useForm<LoginData>();
        const {t} = useTranslation()

        type Invite = {
            id: string,
            role: string,
            createdAt: string,
            acceptedAt: string,
            expiresAt: string,
            explicitContent: boolean
        }

        useEffect(()=>{
            axios.get(apiURL+"/users/invites/"+params.id).then((res)=>{
                setInvite(res.data)
                })
                .catch(()=>{
                    setErrored(true)
                })
        },[])

        if(!invite&& !errored){
            return <Loading/>
        }

        const onSubmit: SubmitHandler<LoginData> = (data)=>{
            axios.post(apiURL+"/users/", {
                username: data.username,
                password: data.password,
                inviteId: params.id
            }).then(()=>{
                enqueueSnackbar(t('account-created'), {variant: "success"})
            }).catch(()=>{
                enqueueSnackbar(t('password-too-weak'), {variant: "error"})
            })
        }

        if (!invite) {
            return <Loading/>
        }

        return <section className="bg-gray-900 h-full">
            <div className="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
                <a href="#" className="flex items-center mb-6 text-2xl font-semibold text-white">
                    <i className="fa-solid fa-music mr-5"></i>
                    PodFetch
                </a>
                <div
                    className="w-full rounded-lg shadow border md:mt-0 sm:max-w-md xl:p-0 bg-gray-800 border-gray-700">
                    <div className="p-6 space-y-4 md:space-y-6 sm:p-8">
                        <h1 className="text-xl font-bold leading-tight tracking-tight md:text-2xl text-white">
                            {t('create-account-podfetch')}
                        </h1>
                        <div className="grid place-items-center">
                <form onSubmit={handleSubmit(onSubmit)}>
                <div className="grid grid-cols-2 gap-5 text-white">
                    <div>
                        {t('role')}
                    </div>
                    <div>
                        {invite.role}
                    </div>
                    <div>
                        {t('created')}
                    </div>
                    <div>
                        {formatTime(invite.createdAt)}
                    </div>
                    <div>
                        {t('expires-at')}
                    </div>
                    <div>
                        {formatTime(invite.expiresAt)}
                    </div>
                    <div>
                        {t('explicit-content')}
                    </div>
                    <div>
                        {invite.explicitContent?<i className="fa-solid fa-check"/>:<i className="fa-solid fa-times"></i>}
                    </div>
                    <label htmlFor="username"
                           className="block pt-2.5 pb-2.5 text-white">{t('username')!}</label>
                    <input type="username" {...register('username', {required: true})} id="username"
                           autoComplete="username"
                           className="border sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
                           placeholder={t('your-username')!}/>

                    <label htmlFor="password"
                           className="block pt-2.5 pb-2.5 text-white">{t('password')}</label>
                    <input type="password" id="password" autoComplete="current-password"
                           placeholder="••••••••" {...register('password', {required: true})}
                           className="border sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"/>
                    </div>
                    <button type="submit" className="text-center bg-blue-700 w-full mt-5 pt-1 pb-1 text-white">Absenden</button>
                </form>
    </div>
                    </div>
                </div>
            </div>
        </section>
}
