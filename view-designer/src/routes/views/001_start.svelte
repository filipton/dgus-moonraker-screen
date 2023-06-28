<script lang="ts">
    import Header from "$lib/components/Header.svelte";

    export let preview: boolean;
</script>

<Header {preview} />
<h1 class="text-center text-4xl font-bold text-white my-auto">
    Welcome to the <span class="text-red-800">Svelte</span> GameBoy Emulator!
</h1>
