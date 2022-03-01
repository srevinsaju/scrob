
import { getToken, login, register, isLoggedIn } from './auth'
import { parseUserId } from './utils'
import { UserPlayerLocalLyrics, UserPlayerLocalCurrentSong, ConnectLastFmToken } from "./api"
import { Song } from './types'
import { ListenSongChanges } from './ws'
import { players } from "./ws"

let lastPlayedSong: Song = {
    track: "",
    artist: "",
    source: "",
}
let songChanged: boolean = true

function registerHtmlCallback() {
    let userId = (<HTMLInputElement>document.getElementById('userId')).value
    let password = (<HTMLInputElement>document.getElementById('password')).value
    let confirmedPassword = (<HTMLInputElement>document.getElementById('confirmPassword')).value
    if (password != confirmedPassword) {
        alert('Passwords do not match')
        return
    }
    let parsed = parseUserId(userId)

    register(parsed.username, password, parsed.hostname, function() {
        window.location.replace("/login");
    }, 
    function() {
        alert('Registration failed')
    })   
}



function loginHtmlCallback() {
    let userId = (<HTMLInputElement>document.getElementById('userId')).value
    let password = (<HTMLInputElement>document.getElementById('password')).value
    let parsed = parseUserId(userId)
    login(parsed.username, password, parsed.hostname, function() {
        window.location.replace("/");
    }, function() {

    alert('Login failed')

    })   
}

function registerRegisterButtonCallback() {
    (<HTMLButtonElement>document.getElementById('submitButton')).onclick = registerHtmlCallback
}

function registerLoginButtonCallback() {
    (<HTMLButtonElement>document.getElementById('submitButton')).onclick = loginHtmlCallback
}

function playerEditorClickHandler(event: Event) {
    let targetElement = document.getElementById('editPlayer')
    // @ts-ignore:
    postMessage('playerChangeReset')
    console.log("Requesting backend to fallback to autodetection.")
    targetElement.textContent = "Listening to songs"
    targetElement.removeEventListener('click', playerEditorClickHandler)
}

function postMessage(message: string) {
    fetch(`http://localhost:8000/events/status/ ${message}`).catch(function(e) {
        console.log("Failed sending state", e)
    })
}

function onDefaultPageLoadCallback() {
    let scrobbleSwitch = <HTMLInputElement>document.getElementById('scrobbleSwitch');
    scrobbleSwitch.addEventListener('change', function(evt) {
        if (scrobbleSwitch.checked) {
            // @ts-ignore: 
            postMessage('scrobbleSwitchEnabled');
        } else {
            // @ts-ignore: 
            postMessage('scrobbleSwitchDisabled');
        }

    })
    let discordSwitch = <HTMLInputElement>document.getElementById('discordSwitch');
    discordSwitch.addEventListener('change', function(evt) {
        if (discordSwitch.checked) {
            // @ts-ignore: 
            postMessage('discordSwitchEnabled');
        } else {
            // @ts-ignore: 
            postMessage('discordSwitchDisabled');
        }

    })
    let playerName = <HTMLInputElement>document.getElementById('playerName');
    let playerBlock = document.getElementById('playerBlock');
    let playerIcon = (document.getElementById('playerIcon') as HTMLImageElement);
    let playerEditor = document.getElementById('editPlayer');
    let lastChosenCustomPlayer = "";
    playerName.addEventListener('keyup', function(evt) {
        playerBlock.classList.add('is-loading')
        
        for (let i = 0; i < players.length; i++) {
            if (playerName.value.toLowerCase() == players[i].displayName.toLowerCase() || playerName.value.toLowerCase() == players[i].name.toLowerCase()) {
                playerName.value = players[i].displayName;
                playerIcon.src = players[i].image;
                if (lastChosenCustomPlayer != players[i].name) {
                    lastChosenCustomPlayer = players[i].name;
                     // send a request to the backend to change the configured player
                    // @ts-ignore:
                    postMessage(`playerChangeRequested:${players[i].name}`)
                    playerEditor.textContent = "Overriding player details. Reset?"
                    
                    playerEditor.addEventListener('click', playerEditorClickHandler)
                    break;
                }
            } 
        }

    })
    playerName.addEventListener('change', function(evt) {
        playerBlock.classList.remove('is-loading')
    })
    playerName.addEventListener('focusout', function(evt) {
        playerBlock.classList.remove('is-loading')
    })
}

function onTokenPageLoadCallback() {
    document.getElementById('authCode').textContent = getToken()
}

function navBarSetup() {
    let navBar =  <HTMLButtonElement>document.getElementsByClassName('navbar-burger').item(0)
    navBar.onclick = function() {
        navBar.classList.toggle('is-active')
        document.getElementsByClassName('navbar-menu').item(0).classList.toggle('is-active')
    }
    
}

function spotifyAuthorizeTokenCallback() {
    let title = document.getElementsByClassName('title').item(0)
    let subtitle = document.getElementsByClassName('subtitle').item(0)
    const urlParams = new URLSearchParams(window.location.search);
    let code = urlParams.get("code")
    if (code == "") {
        title.textContent = "🎶 Looks like something went wrong 👀"
        subtitle.textContent = "Please try again."
        return
    }
    title.textContent = "🎶 Your Lyrix account is connected with Spotify! 🎶"
    subtitle.textContent = "The spotify token was received, and was successfully updated. Your songs should now be visible over Lyrix ✨"
    
}

function lastFMAuthorizeTokenCallback() {
    let title = document.getElementsByClassName('title').item(0)
    let subtitle = document.getElementsByClassName('subtitle').item(0)
    let button: HTMLLinkElement = <HTMLLinkElement>document.getElementById('authorize')

    title.textContent = "🎶 Click on the button below to Connect to Last.fm! 🎶"
    subtitle.textContent = "The lastFM authorization was received, and was successfully updated. Your songs should now be visible over Lyrix ✨"
    ConnectLastFmToken(function(res: {redirect: string}) {
        button.textContent = "Connect to Last.fm"
        button.href = res.redirect
    }, function() {
        console.log("Couldn't connect to last.fm")
        button.textContent = "Couldn't connect to Last.fm"
    })
    
}


function loadSong() {
    // fetches the current song that the user is listening
    // if the user's song has changed compared to that reflected 
    // on the UI, the UI will be updated
    // and the lyrics will be fetched
    UserPlayerLocalCurrentSong(function(res: Song) {
        if (lastPlayedSong.track == res.track && lastPlayedSong.artist == res.artist) {
            songChanged = false
        } else {
            lastPlayedSong.track = res.track
            lastPlayedSong.artist = res.artist
            songChanged = true
            let title = document.getElementsByClassName('title').item(0)
            let artist = document.getElementsByClassName('subtitle').item(0)
            if (res.track == "" || res.artist == "") {
                title.textContent = "Scrob"
                artist.textContent = "You are currently not playing any song"
            } else {
                title.textContent = res.track 
                artist.textContent = res.artist
            }
            
        }
    

    }, function(res: string) {
        console.log("Couldn't fetch the current listening song,", res)
    })
}

function loadLyrics() {
    // do not do anything if the song hasn't changed
    // we can avoid a lot of API calls if we do that
    if (!songChanged) { return }
    if (lastPlayedSong.track == "" || lastPlayedSong.artist == "") { return }
    
    let lyrics = document.getElementById('lyrics')
    UserPlayerLocalLyrics(function(res :string) {
        console.log(res)
        lyrics.textContent = res    
        songChanged = false
        
    }, function() {
        lyrics.textContent = "Couldn't fetch the lyrics."
    })
    
}

export function registerAllCallbacks() {

    switch (window.location.pathname) {
        case "/register/":
            registerRegisterButtonCallback()
            break; 
        case "/login/":
            registerLoginButtonCallback()
            break;
        case "/token/":
            onTokenPageLoadCallback()
            break;
        case "/authorize/spotify/":
            spotifyAuthorizeTokenCallback()
            break;
        case "/authorize/lastfm/":
            lastFMAuthorizeTokenCallback()
            break;
        case "/":
            onDefaultPageLoadCallback()
            navBarSetup()
            ListenSongChanges()

            break

        default:
            onDefaultPageLoadCallback()
            navBarSetup()
            break;
    }
}
