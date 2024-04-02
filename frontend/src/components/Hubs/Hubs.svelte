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
    <div class="hubs">
        {#each hubs as hub}
            <div class="hub-button">
                <img class="hub-image" src={hub.imageurl} alt={hub.name} />
            </div>
        {/each}
    </div>
</main>

<style>

  .hubs {
    width: 10vw;
    display: flex;
    flex-direction: column;
    gap: 1rem; /* Spacing between buttons */
    background-color: transparent;
  }
  /*#222831 #31363F #76ABAE #EEEEEE*/
  .hub-button {
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 50%;
    width: 6vw;
    height: 6vw;
    aspect-ratio: 1;
    margin: 0rem 0.5rem;
    background-color: #313631;
    cursor: pointer;
    transition: background-color 0.2s ease;
    position: relative; /* Enable positioning of pseudo-elements */
  }

  .hub-button:hover {
    background-color: rgba(118, 171, 174, 0.5); /* Semi-transparent #76ABAE */
  }

  .hub-button img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    border-radius: 50%;
    }

    .hub-button::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(118, 171, 174, 0.8); /* Semi-transparent #76ABAE */
    border-radius: 50%;
    transition: background-color 0.2s ease;
    opacity: 0; /* Initially invisible */
    }

    .hub-button:hover::before {
  opacity: 0.5; /* Semi-transparent background on hover */
}
</style>
