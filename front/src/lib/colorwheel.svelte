<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { colorFromHue } from "./hue.svelte";

    export let posts: any;
    $: colors = posts.map((post) => colorFromHue(post.hue));

    let angle = writable(0);
    let currentpost = writable(0);
    $: {
        let segments = 360 / posts.length;
        let index = Math.floor((($angle + 360) % 360) / segments);
        console.log("Angle: ", $angle);
        console.log("Segments: ", segments);
        currentpost.set(index);
        console.log("Index: ", index);
    }

    // Calculate svg path for a pie slice of the color wheel
    function getPath(index, total, radius) {
        const angle = (index / total) * Math.PI * 2;
        const endAngle = ((index + 1) / total) * Math.PI * 2;

        const startX = 50 + radius * Math.cos(angle);
        const startY = 50 + radius * Math.sin(angle);
        const endX = 50 + radius * Math.cos(endAngle);
        const endY = 50 + radius * Math.sin(endAngle);
        const largeArc = endAngle - angle > Math.PI ? 1 : 0;

        return `M 50 50 L ${startX} ${startY} A ${radius} ${radius} 0 ${largeArc} 1 ${endX} ${endY} Z`;
    }

    let dragging = false;
    let startX = 0;
    let startY = 0;
    function onMouseDown(event) {
        dragging = true;
        startX = event.clientX;
        startY = event.clientY;
        event.preventDefault(); // Prevent text selection
    }

    function onMouseMove(event) {
        if (dragging) {
            const dx = event.clientX - startX;
            const dy = event.clientY - startY;
            const distance = Math.sqrt(dx * dx + dy * dy);
            const scaledDistance = (distance / 3) % 360;

            angle.set(scaledDistance);
        }
    }

    function onMouseUp() {
        dragging = false;
    }

    onMount(() => {
        window.addEventListener("mouseup", onMouseUp);
        window.addEventListener("mousemove", onMouseMove);
        return () => {
            window.removeEventListener("mouseup", onMouseUp);
            window.removeEventListener("mousemove", onMouseMove);
        };
    });
</script>

<svg
    viewBox="0 0 100 100"
    style="width: 450px; height: 450px; transform: rotate({$angle}deg);"
    on:mousedown={onMouseDown}
>
    {#each colors as color, index}
        <path d={getPath(index, posts.length, 50)} fill={color} />
    {/each}
</svg>

<div class="active-post">
    <p>{posts[$currentpost].text}</p>
    <p>{posts[$currentpost].author}</p>
</div>

<div class="pointer"></div>

<style>
    p {
        font-size: 17pt;
        font-weight: 400;
    }
    svg {
        transition: transform 0.3s linear;
        border-radius: 50%;
        overflow: hidden;
        position: fixed;
        left: 0%;
        top: 23%;
        cursor: grab;
    }
    .pointer {
        width: 20px;
        height: 20px;
        border-radius: 50%;
        background-color: white;
        position: fixed;
        left: 30%;
        top: 50%;
    }
    svg:active {
        cursor: grabbing;
    }
    .active-post {
        width: 30%;
        position: absolute;
        top: 20%;
        left: 50%;
        transform: translateX(-50%);
        text-align: center;
    }
</style>
