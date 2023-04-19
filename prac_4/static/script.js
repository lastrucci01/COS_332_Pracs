function openPopup() {
    document.getElementById("appointment_popup").style.display = "flex";
}

function closePopup() {
    document.getElementById("appointment_popup").style.display = "none";
    event.preventDefault();
}

function submitSearch() {
    var form = document.getElementById("search");
    form.submit();
}