var lFollowX = 0;
var lFollowY = 0;
var x = 0;
var y = 0;
var range = 1000;
var friction = 1 / 50;

var appWindow = window.__TAURI__.window.appWindow

function moveBackground() {
    x += (lFollowX - x) * friction;
    y += (lFollowY - y) * friction;
    translate = 'translate(' + x + 'px, ' + y + 'px) scale(1.1)';
    document.getElementById('bg').style.transform = translate;
    window.requestAnimationFrame(moveBackground);
}

function handleMouseMove(e) {
    var lMouseX = Math.max(-range, Math.min(range, window.innerWidth / 2 - e.clientX));
    var lMouseY = Math.max(-range, Math.min(range, window.innerHeight / 2 - e.clientY));
    lFollowX = (7 * lMouseX) / range;
    lFollowY = (7 * lMouseY) / range;
}

function load() {
    const animated = document.getElementById('raot-button');
    animated.style.visibility = 'hidden';

    animated.addEventListener('animationend', () => {
        animated.style.visibility = 'visible';
    });
    
    document.getElementById('min-btn').addEventListener('click', () => appWindow.minimize())
    document.getElementById('close-btn').addEventListener('click', () => appWindow.close())
    window.onmousemove = handleMouseMove;
    moveBackground();
}