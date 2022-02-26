import { detectSchemeFromHostname } from "./utils";

const axios = require('axios').default;


export function isLoggedIn(): boolean {
    let lKeys = ["token", "username", "hostname"] 
    for (const key in lKeys) {
        if (localStorage.getItem(lKeys[key]) === null) {
            return false;
        }
        // TODO, what if the person's name is null?
        if (localStorage.getItem(lKeys[key]) === "null") {
            return false;
        }
    }
    return true;
}

export function getToken(): string {
    return "/start " + localStorage.getItem("username") + ":" + localStorage.getItem("hostname") + ":" + localStorage.getItem("token");
}

export function login(
    username: string, 
    password: string,
    hostname: string, 
    success: Function, 
    error: Function)
{
    console.log(`Sending login request to ${hostname}`)
    let scheme = detectSchemeFromHostname(hostname) 
    axios.post(`${scheme}://${hostname}/login`, 
    {
        "username": username, 
        "password": password
    }).then((res: { data: { token: string; }; }) => {
        localStorage.setItem('token', res.data.token);
        localStorage.setItem('username', username);
        localStorage.setItem('hostname', hostname);
        success()
    }).catch((err: any) => {
        console.log(`Failed to send login request: ${err}`);
        error(err)
    })
}

export function register(
    username: string, 
    password: string, 
    hostname: string, 
    success: Function, 
    error: Function)
{
    axios.post(`https://${hostname}/register`, 
    {
        "username": username, 
        "password": password,
    }).then((res: { data: { token: string; }; }) => {
        success()
    }).catch((err: any) => {
        console.log(`Failed to send register request: ${err}`);
        error(err)
    })
}