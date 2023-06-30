export async function cropImageArea(imageBlob: any, canvasWidth: number, canvasHeight: number, left: number, top: number, width: number, height: number) {
    //create temp canvas and set image data
    let canvas1 = document.createElement('canvas');
    let ctx1 = canvas1.getContext('2d');
    canvas1.width = canvasWidth;
    canvas1.height = canvasHeight;

    let img = new Image();
    img.src = URL.createObjectURL(imageBlob);

    let done = false;
    img.onload = function() {
        ctx1?.drawImage(img, 0, 0);
        done = true;
    }

    // wait for image to load
    while (!done) {
        console.log('waiting for image to load');
        await sleep(10);
    }

    // crop image
    let canvas2 = document.createElement('canvas');
    let ctx2 = canvas2.getContext('2d');
    canvas2.width = width;
    canvas2.height = height;
    ctx2?.drawImage(canvas1, left, top, width, height, 0, 0, width, height);

    return canvas2.toDataURL('image/jpeg', 1.0);
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
