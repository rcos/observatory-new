const sharedOpts = {
  plugins: ["dayGrid", "list"],
  themeSystem: "bootstrap"
};

document.addEventListener("DOMContentLoaded", function() {
  var calendarEl = document.getElementById("calendar");

  if (calendarEl) {
    var calendar = new FullCalendar.Calendar(calendarEl, {
      ...sharedOpts,
      header: {
        right: "dayGridMonth,listYear today prev,next"
      },
      defaultView: "dayGridMonth",
      events: {
        url: "/calendar.json",
        editable: false
      },
      eventDataTransform: data => ({
        ...data,
        url: "/calendar/" + data.id
      })
    });

    calendar.render();
  }

  var newsEl = document.getElementById("news");

  if (newsEl) {
    var news = new FullCalendar.Calendar(newsEl, {
      ...sharedOpts,
      header: {
        right: "listYear,dayGridMonth today prev,next"
      },
      defaultView: "list",
      events: {
        url: "/news.json",
        editable: false
      },
      eventDataTransform: data => ({
        ...data,
        start: data.happened_at,
        end: data.happend_at,
        url: "/news/" + data.id
      })
    });

    news.render();
  }
});
