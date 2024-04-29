<script lang="ts">
    import { user_id } from "../stores/user.js";
    import { get } from "svelte/store";
    export let author: string;
    export let following: boolean;
    export let followee: number;

    async function followButton() {
        let follower = get(user_id);

        if (!following) {
            await fetch("http://127.0.0.1:8000/post/follow", {
                method: "POST",
                headers: {
                    "Content-Type": "text/plain",
                },
                body: JSON.stringify({ follower, followee }),
            })
                .then((res) => {
                    if (res.ok) {
                        following = true;
                    }
                })
                .catch((err) => {
                    console.error(err);
                    following = false;
                });
        } else {
            await fetch("http://127.0.0.1:8000/delete/follow", {
                method: "DELETE",
                headers: {
                    "Content-Type": "text/plain",
                },
                body: JSON.stringify({ follower, followee }),
            })
                .then((res) => {
                    if (res.ok) {
                        following = false;
                    }
                })
                .catch((err) => {
                    console.error(err);
                    following = true;
                });
        }
    }
</script>

{#if $user_id !== 0}
    <button class="button enabled" on:click={followButton}>
        {#if following}
            following
        {:else}
            follow
        {/if}
    </button>
{:else}
    <button class="button disabled">log in!</button>
{/if}

<style>
    .button {
        width: 6rem;
        height: 2.5rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        border-radius: 1rem;
        padding: 1rem;
        margin-bottom: 1rem;
        text-decoration: none;
        background-color: transparent;
        font-family: "Spectral", serif;
    }

    .button:hover {
        background-color: rgb(255, 255, 255, 0.1);
    }

    .enabled {
        border: 1px solid white;
        color: white;
    }

    .disabled {
        border: 1px solid #aaa;
        color: #aaa;
    }
</style>
