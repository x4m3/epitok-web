/**
 * Set all students as the same presence status
 * @param presence - student login
 */
function setAll(presence) {
    for (let field of document.getElementsByTagName("form")[0].elements) {
        if (field.value === presence) {
            field.checked = true;
            field.parentElement.classList.add("active");
        } else {
            field.checked = false;
            field.parentElement.classList.remove("active");
        }
    }
}

/**
 * Set all students marked as None as the same presence status
 * @param presence - student login
 */
function setRemaining(presence) {
    let usersToChange = [];
    // check if none marked -> save login in array, set as unmarked
    for (let field of document.getElementsByTagName("form")[0].elements) {
        if (field.value === "None" && field.checked === true) {
            usersToChange.push(field.name);
            field.checked = false;
            field.parentElement.classList.remove("active");
        }
    }
    // for each element in array: if presence & login match -> set as marked
    usersToChange.forEach(function (item) {
        for (let field of document.getElementsByTagName("form")[0].elements) {
            if (field.name === item && field.value === presence) {
                field.checked = true;
                field.parentElement.classList.add("active");
                break;
            }
        }
    });
}

/**
 * Set specific student a presence status
 * @param login - student login
 * @param presence - presence as seen in epitok (name of the enum value)
 * @returns bool - presence set or not
 *
 * note: if presence has already been set for student, returns false
 * returns true if presence was not set for student
 */
function setStudent(login, presence) {
    let foundStudent = false;
    for (let field of document.getElementsByTagName("form")[0].elements) {
        if (field.name === login) {
            if (field.value === presence) {
                if (field.checked === true) {
                    return false;
                }
                field.checked = true;
                field.parentElement.classList.add("active");
                foundStudent = true;
            } else {
                field.checked = false;
                field.parentElement.classList.remove("active");
            }
        }
    }
    return foundStudent;
}

/**
 * Remove spinner from save button
 */
function saveButtonCallback() {
    $("#save span").remove();
    $("#save").attr("disabled", false).html("Save");
}

/**
 * When save button is clicked
 */
$("#save").click(function () {
    $("#callback-save div").alert('close');
    $("#save").html('<span class="spinner-grow spinner-grow-sm mr-2" role="status" aria-hidden="true"></span>Saving...').attr("disabled", true);
    let URL = window.location.href + "/save";
    fetch(URL, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify($('form').serializeArray()),
    })
        .then(function (response) {
            if (!response.ok) {
                response.json().then(data => {
                    $("#callback-save").html('<div class="alert alert-danger alert-dismissible fade show" role="alert"><strong>Could not save changes</strong> ' + data + '.<button type="button" class="close" data-dismiss="alert" aria-label="Close"><span aria-hidden="true">&times;</span></button></div>');
                    saveButtonCallback();
                });
                throw Error("this is normal, by throwing this error the error alert will show up.");
            }
            return response;
        }).then(function () {
        $("#callback-save").html('<div class="alert alert-success alert-dismissible fade show" role="alert"><strong>Changes saved</strong> You can do something else now.<button type="button" class="close" data-dismiss="alert" aria-label="Close"><span aria-hidden="true">&times;</span></button></div>');
        saveButtonCallback();
    }).catch(function (error) {
        $("#callback-save").html('<div class="alert alert-danger alert-dismissible fade show" role="alert"><strong>Could not save changes</strong> ' + error + '<button type="button" class="close" data-dismiss="alert" aria-label="Close"><span aria-hidden="true">&times;</span></button></div>');
        saveButtonCallback();
    });
});

/**
 * Define actions for set-all and set-remaining buttons
 */
$("#set-all-present").click(function () {
    setAll("Present");
});

$("#set-all-missing").click(function () {
    setAll("Missing");
});

$("#set-all-not-applicable").click(function () {
    setAll("NotApplicable");
});

$("#set-all-none").click(function () {
    setAll("None");
});

$("#set-remaining-present").click(function () {
    setRemaining("Present");
});

$("#set-remaining-missing").click(function () {
    setRemaining("Missing");
});

$("#set-remaining-not-applicable").click(function () {
    setRemaining("NotApplicable");
});

/**
 * QR Scanner from nimiq
 * https://github.com/nimiq/qr-scanner/tree/e8a77de
 * MIT
 */
import QrScanner from "/static/qr-scanner/qr-scanner.min.js";

QrScanner.WORKER_PATH = "/static/qr-scanner/qr-scanner-worker.min.js";

/**
 * Check if it's possible to scan QR codes
 */
$(async function () {
    if (await QrScanner.hasCamera() === false) {
        $("#scan").attr("disabled", true).html("Cannot scan QR Codes :(");
    }
});

function showScannedStudentAlert(student) {
    $("#scanned-student-alert").html('<div class="alert alert-success alert-dismissible fade show" role="alert">Student <b>' + student + '</b> has been set as <b>Present</b>.<button type="button" class="close" data-dismiss="alert" aria-label="Close"><span aria-hidden="true">&times;</span></button></div>');
}

/**
 * When QR code is detected
 */
const qrScanner = new QrScanner(document.getElementsByTagName("video")[0], function (scannedString) {
    if (setStudent(scannedString, "Present") === true) {
        // Close current alert if element is present
        if ($("#scanned-student-alert div").length) {
            $("#scanned-student-alert div").alert('close');
        }

        // Check if alert is already present
        if ($("#scanned-student-alert div").length) {
            // Wait for previous alert to be closed (wait for css transitions)
            $("#scanned-student-alert div").on("closed.bs.alert", function () {
                showScannedStudentAlert(scannedString);
            });
        } else {
            // If alert is not already present
            showScannedStudentAlert(scannedString);
        }
    }
});

/**
 * Start scanning for QR codes
 */
async function startScanning() {
    $("#save").attr("disabled", true);
    $("#set-all-present").attr("disabled", true);
    $("#button-set-all-dropdown").attr("disabled", true);
    $("#set-remaining-missing").attr("disabled", true);
    $("#button-set-remaining-dropdown").attr("disabled", true);
    $("#scan").html("Stop scanning").addClass("scanning");
    $("video").show();
    qrScanner.start();
    if (await qrScanner.hasFlash() === true) {
        await qrScanner.turnFlashOn(); // turn on flashlight if supported
    }
}

/**
 * Stop scanning for QR codes
 */
function stopScanning() {
    qrScanner.stop();
    qrScanner.destroy();
    $("video").hide();
    $("#save").attr("disabled", false);
    $("#set-all-present").attr("disabled", false);
    $("#button-set-all-dropdown").attr("disabled", false);
    $("#set-remaining-missing").attr("disabled", false);
    $("#button-set-remaining-dropdown").attr("disabled", false);
    $("#scan").html("Scan QR codes").removeClass("scanning");
}

/**
 * Action when scan button is clicked
 */
$("#scan").click(function () {
    // Check if scanning already
    if ($("#scan").hasClass("scanning")) {
        stopScanning();
    } else {
        startScanning();
        // TODO: handle promise?
    }
});
