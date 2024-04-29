<script lang="ts">
    import { user_id } from "../stores/user.js";
</script>

{#if $user_id != 0}
    <a class="button enabled" href="/new"> new </a>
{:else}
    <div class="button disabled">log in!</div>
{/if}

<style>
    @import "./button_style.css";

    .button {
        right: 20%;
        top: 50%;
    }
</style>
