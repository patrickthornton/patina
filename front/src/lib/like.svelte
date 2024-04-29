<script lang="ts">
    import { user_id } from "../stores/user.js";
    export let post: number;

    let liked = false;

    async function like() {
        let user = $user_id;
        liked = !liked;
        await fetch("http://127.0.0.1:8000/post/like", {
            method: "POST",
            headers: {
                "Content-Type": "text/plain",
            },
            body: JSON.stringify({ post, user, liked }),
        }).catch((err) => console.error(err));
    }
</script>

<div class="like" on:click={like}>{liked ? "\u2764" : "\u2763"}</div>

<style>
    .like {
        padding: none;
        align-self: flex-end;
        font-size: 20pt;
    }
</style>
