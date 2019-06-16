/*
 * Project wide JavaScript imported into base.html
 */

// On page load
// We use an event listener so we don't override window.onload
document.addEventListener('DOMContentLoaded', () => {

    // Any button with type 'delete' will make a DELETE request to the
    // provided `action` or the same page
    document.querySelectorAll("button[type=\"delete\"]").forEach(del => {
        del.addEventListener('click', (e) => {
            let url = del.getAttribute("action") || window.location;
            if (confirm("Are you sure?")) {
                fetch(url, {
                    method: 'DELETE'
                }).then((res) => window.location = res.url)
            }
        });
    });

    // Rows in tables can be clicked on to navigate to them
    document.querySelectorAll("tr.clickable-row").forEach(row => {
        row.addEventListener('click', (e) => {
            window.location = row.dataset.href;
        });
    });

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
