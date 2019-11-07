    
    document.addEventListener('DOMContentLoaded', function () {
        var calendarEl = document.getElementById('calendar');

        var calendar = new FullCalendar.Calendar(calendarEl, {
            plugins: ['dayGrid', 'list'],
            header: {
                right: 'dayGridMonth,listWeek today prev,next'
            },
            defaultView: 'dayGridMonth',
            themeSystem: 'bootstrap',
            events: {
                url: "/calendar.json",
                editable: false
            },
            eventDataTransform: (data) => ({
                ...data,
                url: "/calendar/" + data.id
            })

        });

        calendar.render();
    });
