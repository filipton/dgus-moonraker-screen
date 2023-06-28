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
            a.href = canvas.toDataURL("image/jpeg");
            a.download = module.name + ".jpg";
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
        a.href = canvas.toDataURL("image/jpeg");
        a.download = currentModule.name + ".jpg";
        a.click();
    }
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
