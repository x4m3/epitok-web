/* autologin link validation */
(function () {
        "use strict";
        window.addEventListener("load", function () {
            let forms = document.getElementsByClassName("needs-validation");
            Array.prototype.filter.call(forms, function (form) {
                form.addEventListener("submit", function (event) {
                    form.classList.add("was-validated");
                    if (form.checkValidity() === true) {
                        $("#signin-button").html('<span class="spinner-grow spinner-grow-sm mr-2" role="status" aria-hidden="true"></span>Signing in...').attr("disabled", true);
                    } else {
                        event.preventDefault();
                        event.stopPropagation();
                    }
                }, false);
            });
        }, false);
    }
)();