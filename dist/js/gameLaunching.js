var invoke = window.__TAURI__.invoke

function saveChanges() {
    //config.raot.path = document.getElementById('raot-input').value;
}
        
function showRaotScreen() {
    var mainScreen = document.getElementById('main-screen');
    mainScreen.classList.remove('slide-in-top');
    mainScreen.classList.add('slide-out-top');

    var raotScreen = document.getElementById('raot-screen');
    raotScreen.classList.remove('hidden-screen');
    raotScreen.classList.remove('slide-out-bottom');
    raotScreen.classList.add('raot-screen-visible');
    raotScreen.classList.add('slide-in-bottom');
}

function executeInjectRaot() {
    //invoke('save_changes')
    invoke('launch_raot')
}

function raotBackToMenu() {
    var raotScreen = document.getElementById('raot-screen');
    raotScreen.classList.add('slide-out-bottom');
    window.setTimeout(function() {
        raotScreen.classList.remove('raot-screen-visible');
        raotScreen.classList.add('hidden-screen');
    }, 400);

    var mainScreen = document.getElementById('main-screen');
    mainScreen.classList.remove('slide-out-top');
    mainScreen.classList.add('slide-in-top');
}