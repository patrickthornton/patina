<script lang="ts">
    import { onMount } from "svelte";
    import Timeline from "$lib/timeline.svelte";
    import Title from "$lib/title.svelte";
    import Post from "$lib/interfaces.svelte";

    let posts: Post[] = [];

    var current_time = Math.floor(new Date().getTime() / 1000);
    setInterval(() => {
        current_time = Math.floor(new Date().getTime() / 1000);
    }, 200);

    onMount(async () => {
        posts = await fetch(
            `http://127.0.0.1:8000/gradient/${current_time}`,
        ).then((r) => r.json());
    });
</script>

<body>
    <div class="page">
        <Timeline {posts} />
    </div>
    <Title />
</body>

<style>
    body {
        margin: 0;
        padding: 0;
        font-family: Helvetica, sans-serif;
        color: white;
        background-color: #222;
    }

    .page {
        width: 100vw;
        margin: 0;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
</style>
