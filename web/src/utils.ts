

type ParsedUserId = {
    username: string;
    hostname: string;
}

export function parseUserId(userId: string): ParsedUserId {
    let parts = userId.split('@');
    return {
        username: parts[1],
        hostname: parts[2]
    };
}

export function ParseJwt (token: string): {id: number} {
    let base64Url = token.split('.')[1];
    let base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
    let jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));

    return JSON.parse(jsonPayload);
};

export function detectSchemeFromHostname(hostname: string): string {
    let scheme = "http"
    return scheme
}


export function detectWebsocketSchemeFromHostname(hostname: string): string {
    let scheme = "wss"
    // for development, we can use a local server
    // use ws instead of wss there
    if (hostname.startsWith("localhost") || hostname.startsWith("127.0.0.1")) {
        scheme = "ws"
    }
    return scheme
}
