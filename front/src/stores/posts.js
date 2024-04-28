import { writable } from "svelte/store";
import postdata from "../assets/posts.json";

export const posts = writable(postdata);
