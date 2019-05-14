/*
 * This file is used by edit-project.html and new-project.html
 */

function add_repo() {
    let d = document.getElementById("more-repos");
    let i = document.createElement("input");
    i.setAttribute("type", "url");
    i.setAttribute("name", "");
    i.classList.add("form-control");
    i.classList.add("repo-list");
    d.appendChild(i);
}

document.addEventListener('DOMContentLoaded', () => {
    let form = document.querySelector("form");
    set_repos(form);
    form.addEventListener("change", e => {
        set_repos(form);
    })
})

function set_repos(form) {
    let repolist = form.querySelectorAll(".repo-list");
    console.log(repolist);
    let list = [];
    repolist.forEach(node => list.push(node.value));
    form.elements["repos"].value = JSON.stringify(list);
}