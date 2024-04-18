<script lang="ts">
    import type { Hub } from "../../types";

    let hubs: Hub[] = [
        {
            id: 1,
            name: "S",
            imageurl: "https://picsum.photos/id/64/200",
        },
        {
            id: 2,
            name: "hub",
            imageurl: "https://picsum.photos/id/40/200",
        },
        {
            id: 1,
            name: "S",
            imageurl: "https://picsum.photos/id/237/200",
        },
        {
            id: 2,
            name: "hub",
            imageurl: "https://picsum.photos/id/95/200",
        },
    ];
</script>

<main>
  
<div class="isolate rounded-bl-x1 bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md">
  <div class="hubs">
        {#each hubs as hub}
            <div class="hub-button bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md hover:opacity-25">
                <img class="hub-image" src={hub.imageurl} alt={hub.name} />
            </div>
        {/each}
    </div>
  </div>  
</main>

<style>
  /*#222831 #31363F #76ABAE #EEEEEE*/

  .hubs {
    width: 10%;
    height: 88%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    background-color: transparent;
    justify-content: center;
  
  }
  .hub-button {
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 50%;
    width: 6vw;
    height: 6vw;
    aspect-ratio: 1;
    margin: 3px 0.5rem;
    cursor: pointer;
    transition: background-color 0.2s ease;
    position: relative;
    
  
  }


  .hub-button img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    border-radius: 50%;
    }

</style>
