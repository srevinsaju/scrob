import { GetWebsocketId } from './api'
import { WebsocketAuth, WebsocketSong } from './types'

export const players = [
    {
        "name": "Youtube",
        "image": "https://cdn.discordapp.com/app-icons/944684385372884994/9c631d1afe51fdd12579913f45a807d1.png",
        "displayName": "Youtube",
    },
    {
        "name": "YoutubeMusic",
        "image": "https://cdn.discordapp.com/app-assets/916058337152499763/916058618158280714.png",
        "displayName": "Youtube Music"
    },
    {
        "name": "Spotify",
        "image": "https://cdn.discordapp.com/app-icons/916059119415328788/c63864835f882a5e6046e2d089f31576.png",
        "displayName": "Spotify"
    },
    {
        "name": "Lollypop",
        "image": "https://cdn.discordapp.com/app-icons/939527592552308786/b8a47582fff476057e1c1e0ff78224ca.png",
        "displayName": "Lollypop"
    },
    {
        "name": "GenericMusicPlayer",
        "image": "https://cdn.discordapp.com/app-assets/934740230538407968/934779215369351228.png",
        "displayName": "Generic Music Player"
    }
]

export function ListenSongChanges() {
    let title = document.getElementsByClassName('title').item(0)
    let artist = document.getElementsByClassName('subtitle').item(0)
    let playerName = document.getElementById('playerName') as HTMLInputElement
    let playerIcon = (document.getElementById('playerIcon') as HTMLImageElement)
    let playerEditor = document.getElementById('editPlayer');
    
    let wsEndpoint = `ws://localhost:3012`
    let ws = new WebSocket(wsEndpoint)

    let websocketHandshakeSuccess = false

    ws.onmessage = function (event) {
        if (!websocketHandshakeSuccess) {
            websocketHandshakeSuccess = true
            playerEditor.textContent = "Listening to songs"
        }
        
        let data = JSON.parse(event.data)
        if (data.track == "" || data.artist == "") {
            title.textContent = "Scrob"
            artist.textContent = "You are currently not playing any song."
            return
        }
        
        for (let i = 0; i < players.length; i++) {
            if (data.source == players[i].name) {
                playerName.value = players[i].displayName;
                playerIcon.src = players[i].image;
                break;
            }
        }
        title.textContent = data.track
        artist.textContent = data.artist
    }
    ws.onerror = function (event) {
        console.log(event)
        console.log(`Something went wrong when connecting to wsEndpoint: ${wsEndpoint}`)
    }
}
