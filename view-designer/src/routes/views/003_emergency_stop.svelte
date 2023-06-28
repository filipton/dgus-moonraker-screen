<script lang="ts">
    import Header from "$lib/components/Header.svelte";

    export let preview: boolean;
</script>

<Header {preview} />
<div class="w-full h-[calc(100%-2rem)]">
    <div class="text-center font-bold text-4xl mt-4 text-red-600">
        EMERGENCY STOPPED
    </div>

    <div class="flex justify-center mt-8">
        <a
            class="rounded-lg border-white border-2 border-solid px-4 py-2 font-bold text-6xl text-white bg-gray-800"
            title="BTN 9"
            href="?mod=1"
        >
            RESTART
        </a>
    </div>
</div>
