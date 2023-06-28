<script lang="ts">
    import "../app.css";
    import html2canvas from "html2canvas";
    import type { View } from "../lib/types";

    const screenWidth = 480;
    const screenHeight = 272;
    let screen: HTMLDivElement;

    const modules = import.meta.glob("./views/**.svelte");
    let allModules: View[] = [];
    let currentModule: View;

    let loaded = false;

    for (let module in modules) {
        let moduleNumber = parseInt(
            (module.split("/").pop() || "").split("_")[0]
        );

        let name = (module.split("/").pop() || "").replace(".svelte", "");

        allModules.push({
            id: moduleNumber,
            name: name,
            path: module,
        });
    }
    currentModule = allModules[0];

    async function saveAll() {
        for (let module of allModules) {
            currentModule = module;

            while (!loaded) {
                await new Promise((resolve) => setTimeout(resolve, 10));
            }

            let canvas = await html2canvas(screen, {
                useCORS: true,
            });

            const a = document.createElement("a");
            a.href = CanvasToBMP.toDataURL(canvas);
            a.download = currentModule.name + ".bmp";
            a.click();
            await next();
        }
    }

    async function load() {
        loaded = true;
    }

    async function prev() {
        loaded = false;

        let currentIdx = allModules.findIndex((x) => x == currentModule);
        if (currentIdx > 0) {
            currentModule = allModules[currentIdx - 1];
        }
    }

    async function next() {
        loaded = false;

        let currentIdx = allModules.findIndex((x) => x == currentModule);
        if (currentIdx < allModules.length - 1) {
            currentModule = allModules[currentIdx + 1];
        }
    }

    async function save() {
        if (!loaded) {
            alert("Please wait for the module to load!");
            return;
        }

        const canvas = await html2canvas(screen, {
            useCORS: true,
        });
        const a = document.createElement("a");
        a.href = CanvasToBMP.toDataURL(canvas);
        a.download = currentModule.name + ".bmp";
        a.click();
    }

    // Converted by ChatGPT (from 32bit BMP to 24bit BMP)
    var CanvasToBMP = {
        toArrayBuffer: function (canvas: any) {
            var w = canvas.width,
                h = canvas.height,
                idata = canvas.getContext("2d").getImageData(0, 0, w, h),
                data32 = new Uint32Array(idata.data.buffer),
                stride = Math.floor((24 * w + 31) / 32) * 4, // row length for 24 bit bitmap incl. padding
                pixelArraySize = stride * h, // total bitmap size
                fileLength = 54 + pixelArraySize, // header size + bitmap
                file = new ArrayBuffer(fileLength),
                view = new DataView(file),
                pos = 0,
                x,
                y = h - 1,
                p,
                s = 0,
                v;

            // write file header
            setU16(0x4d42); // BM
            setU32(fileLength); // total length
            pos += 4; // skip unused fields
            setU32(0x36); // offset to pixels

            // DIB header
            setU32(40); // header size
            setU32(w);
            setU32(h >>> 0); // negative = top-to-bottom
            setU16(1); // 1 plane
            setU16(24); // 24-bits (RGB)
            setU32(0); // no compression (BI_RGB, 0)
            setU32(pixelArraySize); // bitmap size incl. padding (stride x height)
            setU32(2835); // pixels/meter h (~72 DPI x 39.3701 inch/m)
            setU32(2835); // pixels/meter v
            pos += 8; // skip color/important colors

            // bitmap data, change order of ABGR to BGR
            while (y > 0) {
                p = 0x36 + y * stride; // offset + stride x height
                x = 0;
                while (x < w * 3) {
                    v = data32[s++]; // get ABGR
                    view.setUint8(p + x, (v >> 16) & 0xff); // set R
                    view.setUint8(p + x + 1, (v >> 8) & 0xff); // set G
                    view.setUint8(p + x + 2, v & 0xff); // set B
                    x += 3;
                }
                y--;
            }

            return file;

            // helper method to move current buffer position
            function setU16(data: any) {
                view.setUint16(pos, data, true);
                pos += 2;
            }
            function setU32(data: any) {
                view.setUint32(pos, data, true);
                pos += 4;
            }
        },

        toBlob: function (canvas: any) {
            return new Blob([this.toArrayBuffer(canvas)], {
                type: "image/bmp",
            });
        },

        toDataURL: function (canvas: any) {
            var buffer = new Uint8Array(this.toArrayBuffer(canvas)),
                bs = "",
                i = 0,
                l = buffer.length;
            while (i < l) bs += String.fromCharCode(buffer[i++]);
            return "data:image/bmp;base64," + btoa(bs);
        },
    };
</script>

<div class="flex justify-center mt-8 flex-col items-center">
    <h1 class="text-center text-4xl font-bold text-black my-auto">
        Current: {currentModule.name}
    </h1>

    <div
        class="border-lime-500 border-solid border-2"
        style="width: {screenWidth + 4}px; height: {screenHeight + 4}px;"
    >
        <div
            bind:this={screen}
            class="w-full h-full"
            style="background-color: black;"
        >
            {#await import(currentModule.path)}
                <p>loading...</p>
            {:then module}
                <svelte:component this={module.default} />

                {#await load()}{/await}
            {:catch}
                <p>Error while loading module!</p>
            {/await}
        </div>
    </div>
</div>

<div class="flex justify-center mt-2">
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={prev}>Prev</button>
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={save}>Save</button>
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={saveAll}
        >Save all</button
    >
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={next}>Next</button>
</div>
