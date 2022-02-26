import { registerAllCallbacks } from './html';

function init() {
    registerAllCallbacks();
}

window.addEventListener('load', function () {
    init()
})

