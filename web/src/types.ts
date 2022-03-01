
export type LastFmRedirect = {
    redirect: string;
}

export type Song = {
    track: string;
    artist: string;
    source: string;
}


export type WebsocketAuth = {
    id: string;
    userId: number;
    hostname: string
}

export type WebsocketSong = {
    track: string;
    lyrics: string;
    artist: string;
}