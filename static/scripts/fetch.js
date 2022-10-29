const API_URL = 'http://localhost:8080';

function applyVisibility(condition, clazz) {
    const visibility = condition ? "visible" : "hidden";

    for (const element of document.getElementsByClassName(clazz)) {
        element.style.visibility = visibility;
    }
}

function updateEntries() {
    fetch(API_URL + '/view')
        .then(res => res.json())
        .then(json => {
            for (const loader of document.getElementsByClassName("loader")) {
                loader.remove();
            }

            const nodes = [];

            for (const post of json) {
                const cardLi = document.createElement("li");

                const author = document.createElement("sub");
                const title = document.createElement("h1");
                const description = document.createElement("p");

                author.innerHTML = "Posted by: <b>" + post["author"] + "</b>";

                title.innerText = post["title"];
                description.innerText = post["description"];

                cardLi.replaceChildren(author, title, description);
                nodes.push(cardLi);
            }

            applyVisibility(nodes.length == 0, "show-on-empty");
            document.getElementById("cards").replaceChildren(...nodes);
        });
}

updateEntries();