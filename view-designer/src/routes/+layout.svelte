<script lang="ts">
    import "../app.css";
    import html2canvas from "html2canvas";

    async function save() {
        //@ts-ignore
        const canvas = await html2canvas(document.querySelector("#screen"), {
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
        <div id="screen" class="w-full h-full" style="background-color: black;">
            <slot />
        </div>
    </div>
</div>

<div class="flex justify-center mt-2">
    <button class="px-4 py-2 bg-gray-400" on:click={save}>save</button>
</div>
