<script>
    import { user_id, user_name } from "../stores/user.js";

    let isHovering = false;
</script>

<div class="user">
    <a
        href="/login"
        class="user-text"
        on:mouseover={() => (isHovering = true)}
        on:mouseleave={() => (isHovering = false)}
        on:focus={() => (isHovering = true)}
        on:click={() => {
            user_id.set(0);
            user_name.set("");
        }}
    >
        {#if $user_id != 0}
            {#if !isHovering}
                {$user_name}
            {:else}
                log out
            {/if}
        {:else}
            log in
        {/if}
    </a>
</div>

<style>
    .user {
        position: fixed;
        left: 1.8%;
        top: 2%;
    }

    .user-text {
        color: white;
        text-decoration: none;
        font-size: 1.5rem;
    }
</style>
