var html_classes = document.getElementsByTagName("html")[0].classList;

if (html_classes.contains("navy") || html_classes.contains("ayu") || html_classes.contains("coal")) {
    document.querySelector(".logo").src="static/images/geoffrey-logo-dark.png"
} else {
    document.querySelector(".logo").src="static/images/geoffrey-logo.png"
}