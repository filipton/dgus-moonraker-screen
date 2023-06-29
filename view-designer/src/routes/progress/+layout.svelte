<script lang="ts">
    import "../../app.css";
    import domtoimage from "dom-to-image";

    import { dev } from "$app/environment";
    import { Bitmap } from "$lib/Bitmap";

    const screenWidth = 480;
    const screenHeight = 272;
    let progressBar: HTMLElement;

    let barPercent = 0;

    async function save() {
        for (let i = 0; i <= 100; i++) {
            barPercent = i;
            await new Promise((resolve) => setTimeout(resolve, 100));

            const a = document.createElement("a");
            a.href = await domtoimage.toJpeg(progressBar, { quality: 0.95 });
            a.download = i + ".jpg";
            a.click();
        }
    }
</script>

{#if dev}
    <div class="flex justify-center mt-8 flex-col items-center">
        <div
            class="border-lime-500 border-solid border-2"
            style="width: {screenWidth + 4}px; height: {screenHeight + 4}px;"
        >
            <div class="w-full h-full" style="background-color: black;">
                <div
                    class="w-5/6 h-6 bg-gray-200 rounded-full dark:bg-gray-700 relative"
                    title="DATA 2029/1"
                    bind:this={progressBar}
                >
                    <div class="absolute w-full text-center text-white">
                        {barPercent}%
                    </div>
                    <div
                        class="h-6 bg-blue-600 rounded-full dark:bg-blue-500"
                        style="width: {barPercent}%"
                    />
                </div>
            </div>
        </div>
    </div>

    <div class="flex justify-center mt-2">
        <button class="px-4 py-2 bg-gray-400 mx-1" on:click={save}>Save</button>
    </div>
{:else}
    <h1 class="text-4xl text-red-800 text-center font-bold">
        This page is only available in development mode!
    </h1>
{/if}
