// taken from https://developer.chrome.com/blog/web-audio-autoplay/#moving-forward
(function () {
    // An array of all contexts to resume on the page
    const audioContextList = [];

    // An array of various user interaction events we should listen for
    const userInputEventNames = [
        'click',
        'contextmenu',
        'auxclick',
        'dblclick',
        'mousedown',
        'mouseup',
        'pointerup',
        'touchend',
        'keydown',
        'keyup',
    ];

    // A proxy object to intercept AudioContexts and
    // add them to the array for tracking and resuming later
    self.AudioContext = new Proxy(self.AudioContext, {
        construct(target, args) {
            const result = new target(...args);
            audioContextList.push(result);
            return result;
        },
    });

    // To resume all AudioContexts being tracked
    function resumeAllContexts(event) {
        let count = 0;

        audioContextList.forEach(context => {
            if (context.state !== 'running') {
                context.resume();
            } else {
                count++;
            }
        });

        // If all the AudioContexts have now resumed then we
        // unbind all the event listeners from the page to prevent
        // unnecessary resume attempts
        if (count == audioContextList.length) {
            userInputEventNames.forEach(eventName => {
                document.removeEventListener(eventName, resumeAllContexts);
            });
        }
    }

    // We bind the resume function for each user interaction
    // event on the page
    userInputEventNames.forEach(eventName => {
        document.addEventListener(eventName, resumeAllContexts);
    });
})();
