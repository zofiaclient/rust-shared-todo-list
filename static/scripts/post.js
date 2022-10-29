const API_URL = "http://127.0.0.1:8080"

function val(id) {
    return document.getElementById(id + "-field").value;
}

function setErrText(text) {
    document.getElementById("error-text").innerText = text;
}

function isValidUsername(name) {
    return /^[A-Za-z0-9_]{1,32}$/.test(name)
}

function isValidTitle(title) {
    return /^[ -~]{1,64}$/.test(title)
}

function post() {
    let params = {
        author: val("username"),
        title: val("title"),
        description: val("description")
    };

    if (!isValidUsername(params.author)) {
        setErrText("Invalid username");
        return;
    }
    
    if (!isValidTitle(params.title)) {
        setErrText("Invalid title");
        return;
    }

    params = new URLSearchParams(params);

    fetch(API_URL + "/post?" + params.toString(), {
        method: 'POST'
    })
        .then(response => response.text())
        .then(text => {
            switch (text) {
                case "Ok":
                    window.location.href = '/';
                    break;

                case "Already exists":
                    setErrText("This post is too similar to others; please try again.");
                    break;
                
                case "Character limit":
                    setErrText("Description exceeds maximum character count");
                    break;
            }
        })
}