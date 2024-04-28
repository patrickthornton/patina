<script lang="ts">
    let liked = false;
    let user = 1; //until we figure out login
    export let post: number;

    async function like() {
        await fetch("http://127.0.0.1:8000/post/like", {
            method: "POST",
            headers: {
                "Content-Type": "text/plain",
            },
            body: JSON.stringify({ post, user }),
        }).catch((err) => console.error(err));

        liked = !liked;
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
