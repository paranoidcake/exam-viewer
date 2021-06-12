document.addEventListener('DOMContentLoaded', () => {
    Array.prototype.slice.call(document.getElementsByClassName("navbar-burger"), 0).forEach(element => {
        element.addEventListener('click', () => {
            const $target = document.getElementById(element.dataset.target);

            element.classList.toggle("is-active");
            $target.classList.toggle("is-active");
            
        });
    });

    Array.prototype.slice.call(document.querySelectorAll("a.navbar-item"), 0).forEach(element => {
        if(element.href === window.location.href) {
            if(!element.classList.contains("logo")) {
                element.classList.toggle("is-active");
            }
        }
    });
});