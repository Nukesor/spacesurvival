var Elm = require('./Main.elm');
var app = Elm.Main.fullscreen({timeZoneOffset: new Date().getTimezoneOffset()});

app.ports.readToken.subscribe(function() {
    let user = {};
    let valid = true;
    ['token', 'id', 'podId'].forEach((key) => {
        let val = localStorage.getItem(key);
        if (val !== null) {
            user[key] = val;
        } else {
            valid = false;
        }
    });
    if (valid) {
        app.ports.receiveToken.send(user);
    }
});

app.ports.saveToken.subscribe(function({token, id, podId}) {
    localStorage.setItem('token', token);
    localStorage.setItem('id', id);
    localStorage.setItem('podId', podId);
});
