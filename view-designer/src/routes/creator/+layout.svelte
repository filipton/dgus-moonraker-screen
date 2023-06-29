<script lang="ts">
    import "../../app.css";
    import domtoimage from "dom-to-image";

    import type { View } from "$lib/types";
    import { dev } from "$app/environment";
    import { navigating, page } from "$app/stores";
    import { goto } from "$app/navigation";
    import { Bitmap } from "$lib/Bitmap";

    const screenWidth = 480;
    const screenHeight = 272;
    let screen: HTMLDivElement;
    let preview: boolean = true;

    const modules = import.meta.glob("../views/**.svelte");
    let allModules: View[] = [];
    let currentModule: View;
    let currentModuleNumber: number = parseInt(
        $page.url.searchParams.get("mod") || "0"
    );

    let loaded = false;

    for (let module in modules) {
        let moduleNumber = parseInt(
            (module.split("/").pop() || "").split("_")[0]
        );

        if (allModules.find((x) => x.id == moduleNumber)) {
            console.error(
                `Module ${moduleNumber} already exists! Skipping ${module}...`
            );
            continue;
        }

        let name = (module.split("/").pop() || "").replace(".svelte", "");

        allModules.push({
            id: moduleNumber,
            name: name,
            path: module,
        });

        if (moduleNumber == currentModuleNumber) {
            currentModule = allModules[allModules.length - 1];
        }
    }

    navigating.subscribe((navigating) => {
        currentModuleNumber = parseInt(
            navigating?.to?.url.searchParams.get("mod") || "-1"
        );
        let found = allModules.find((x) => x.id == currentModuleNumber);
        if (found) {
            currentModule = found;
        }
    });

    async function saveAll() {
        if (preview) {
            alert("Please disable preview mode!");
            return;
        }

        loaded = false;
        for (let module of allModules) {
            currentModule = module;

            while (!loaded) {
                await new Promise((resolve) => setTimeout(resolve, 10));
            }

            const a = document.createElement("a");
            a.href = Bitmap.fromPixelData(
                (await domtoimage.toPixelData(screen)).buffer,
                screenWidth,
                screenHeight
            );
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
            currentModuleNumber = currentModule.id;
            goto(`?mod=${currentModuleNumber}`);
        }
    }

    async function next() {
        loaded = false;

        let currentIdx = allModules.findIndex((x) => x == currentModule);
        if (currentIdx < allModules.length - 1) {
            currentModule = allModules[currentIdx + 1];
            currentModuleNumber = currentModule.id;
            goto(`?mod=${currentModuleNumber}`);
        }
    }

    async function save() {
        if (preview) {
            alert("Please disable preview mode!");
            return;
        }

        if (!loaded) {
            alert("Please wait for the module to load!");
            return;
        }

        const a = document.createElement("a");
        a.href = Bitmap.fromPixelData(
            (await domtoimage.toPixelData(screen)).buffer,
            screenWidth,
            screenHeight
        );
        a.download = currentModule.name + ".bmp";
        a.click();
    }
</script>

{#if dev}
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
                    <svelte:component this={module.default} {preview} />

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
        <button
            class="px-4 py-2 mx-1 {preview ? 'bg-gray-600' : 'bg-gray-400'}"
            on:click={() => (preview = !preview)}>Toggle preview</button
        >
        <button class="px-4 py-2 bg-gray-400 mx-1" on:click={saveAll}
            >Save all</button
        >
        <button class="px-4 py-2 bg-gray-400 mx-1" on:click={next}>Next</button>
    </div>
{:else}
    <h1 class="text-4xl text-red-800 text-center font-bold">
        This page is only available in development mode!
    </h1>
{/if}
