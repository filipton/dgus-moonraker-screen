<script lang="ts">
    import "../app.css";
    import html2canvas from "html2canvas";

    let screen: HTMLDivElement;

    const modules = import.meta.glob("./views/**.svelte");
    let allModules: string[] = [];
    let currentModule: string;

    for (let module in modules) {
        allModules.push(module);
    }
    currentModule = allModules[0];

    async function prev() {
        const index = allModules.indexOf(currentModule);
        if (index > 0) {
            currentModule = allModules[index - 1];
        }
    }

    async function next() {
        const index = allModules.indexOf(currentModule);
        if (index < allModules.length - 1) {
            currentModule = allModules[index + 1];
        }
    }

    async function save() {
        const canvas = await html2canvas(screen, {
            useCORS: true,
        });
        const a = document.createElement("a");
        a.href = canvas.toDataURL("image/jpeg");
        a.download = "screenshot.jpg";
        a.click();
    }
</script>

<div class="flex justify-center mt-8">
    <div
        class="border-lime-500 border-solid border-2"
        style="width: 480px; height: 272px;"
    >
        <div
            bind:this={screen}
            class="w-full h-full"
            style="background-color: black;"
        >
            {#await import(currentModule)}
                <p>loading...</p>
            {:then module}
                <svelte:component this={module.default} />
            {:catch}
                <p>Error while loading module!</p>
            {/await}
        </div>
    </div>
</div>

<!--
<div class="flex justify-center mt-8">
    <div
        class="border-lime-500 border-solid border-2"
        style="width: 480px; height: 272px;"
    >
        <div id="screen" class="w-full h-full" style="background-color: black;">
            <slot />
        </div>
    </div>
</div>
-->

<div class="flex justify-center mt-2">
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={prev}>Prev</button>
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={save}>Save</button>
    <button class="px-4 py-2 bg-gray-400 mx-1" on:click={next}>Next</button>
</div>
