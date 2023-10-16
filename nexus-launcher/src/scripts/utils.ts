function animate(render: (value: number) => void, from: number, to: number, duration: number, timeFx: (time: number) => number): void {
    /* Thanks to simonmysun for creating the animate function
       https://stackoverflow.com/questions/65227973/how-to-convert-jquery-animate-function-to-pure-js
    */

    let startTime: number = performance.now();
    requestAnimationFrame(function step(time: number): void {
        let pTime: number = (time - startTime) / duration;
        if (pTime > 1) pTime = 1;
        render(from + (to - from) * timeFx(pTime));
        if (pTime < 1) {
            requestAnimationFrame(step);
        }
    });
}

export {animate};