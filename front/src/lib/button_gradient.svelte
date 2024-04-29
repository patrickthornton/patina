<script lang="ts">
    import { page } from "$app/stores";

    $: onFollowing = $page.url.pathname === "/following";
</script>

{#if onFollowing}
    <a class="button enabled" href="/"> gradient </a>
{:else}
    <a class="button enabled" href="/following"> following </a>
{/if}

<style>
    @import "./button_style.css";

    .button {
        right: 20%;
        top: 40%;
    }
</style>
