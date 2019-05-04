/*
 * Project wide JavaScript imported into base.html
 */

// On page load
// We use an event listener so we don't override window.onload
document.addEventListener('DOMContentLoaded', () => {

    // Any button with id 'delete_button' will make a DELETE request to the same page
    let del = document.getElementById("delete_button")
    if (del) {
        del.addEventListener('click', (e) => {
            fetch(window.location, {
                method: 'DELETE'
            }).then((res) => window.location = res.url)
        })
    }


    // HTML forms cannot handle PUT requests
    // So this adds the functionality
    document.querySelectorAll("[method='PUT']")
        .forEach(f => {
            f.addEventListener('submit', (e) => {
                e.preventDefault();

                // Convert to application/x-www-form-urlencoded
                const data = new URLSearchParams();
                for (const pair of new FormData(e.target)) {
                    data.append(pair[0], pair[1]);
                }

                fetch(e.target.action, {
                    method: 'PUT',
                    body: data
                }).then((res) => window.location = res.url)
            });
        });
});
