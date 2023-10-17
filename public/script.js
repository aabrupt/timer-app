/*
function timer() {}
var Timer = flight.component(timer);
Timer.attachTo;
/* (() => {
    const timer = document.querySelector("#timer");
    if (timer == null) throw new Error("Timer not found");
    timer.addEventListener("startTimer", function () {
        const now = new Date();
        now.setTime(now.getTime() - (this.currentTime ?? 0));
        const startTime = now.getTime();
        this.timer_id = setInterval(
            (element) => {
                element.currentTime =
                    new Date().getTime() - startTime;
                element.innerText = `${Math.floor(
                    element.currentTime / 1000
                )}s`;
            },
            1000,
            this
        );
    });
    timer.addEventListener("stopTimer", function () {
        clearInterval(this.timer_id);
        this.timer_id = undefined;
    });
})(); */
