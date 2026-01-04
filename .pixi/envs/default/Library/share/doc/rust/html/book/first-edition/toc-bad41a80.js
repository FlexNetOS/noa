// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">1.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="guessing-game.html"><strong aria-hidden="true">2.</strong> Tutorial: Guessing Game</a></li><li class="chapter-item expanded "><a href="syntax-and-semantics.html"><strong aria-hidden="true">3.</strong> Syntax and Semantics</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="variable-bindings.html"><strong aria-hidden="true">3.1.</strong> Variable Bindings</a></li><li class="chapter-item expanded "><a href="functions.html"><strong aria-hidden="true">3.2.</strong> Functions</a></li><li class="chapter-item expanded "><a href="primitive-types.html"><strong aria-hidden="true">3.3.</strong> Primitive Types</a></li><li class="chapter-item expanded "><a href="comments.html"><strong aria-hidden="true">3.4.</strong> Comments</a></li><li class="chapter-item expanded "><a href="if.html"><strong aria-hidden="true">3.5.</strong> if</a></li><li class="chapter-item expanded "><a href="loops.html"><strong aria-hidden="true">3.6.</strong> Loops</a></li><li class="chapter-item expanded "><a href="vectors.html"><strong aria-hidden="true">3.7.</strong> Vectors</a></li><li class="chapter-item expanded "><a href="ownership.html"><strong aria-hidden="true">3.8.</strong> Ownership</a></li><li class="chapter-item expanded "><a href="references-and-borrowing.html"><strong aria-hidden="true">3.9.</strong> References and Borrowing</a></li><li class="chapter-item expanded "><a href="lifetimes.html"><strong aria-hidden="true">3.10.</strong> Lifetimes</a></li><li class="chapter-item expanded "><a href="mutability.html"><strong aria-hidden="true">3.11.</strong> Mutability</a></li><li class="chapter-item expanded "><a href="structs.html"><strong aria-hidden="true">3.12.</strong> Structs</a></li><li class="chapter-item expanded "><a href="enums.html"><strong aria-hidden="true">3.13.</strong> Enums</a></li><li class="chapter-item expanded "><a href="match.html"><strong aria-hidden="true">3.14.</strong> Match</a></li><li class="chapter-item expanded "><a href="patterns.html"><strong aria-hidden="true">3.15.</strong> Patterns</a></li><li class="chapter-item expanded "><a href="method-syntax.html"><strong aria-hidden="true">3.16.</strong> Method Syntax</a></li><li class="chapter-item expanded "><a href="strings.html"><strong aria-hidden="true">3.17.</strong> Strings</a></li><li class="chapter-item expanded "><a href="generics.html"><strong aria-hidden="true">3.18.</strong> Generics</a></li><li class="chapter-item expanded "><a href="traits.html"><strong aria-hidden="true">3.19.</strong> Traits</a></li><li class="chapter-item expanded "><a href="drop.html"><strong aria-hidden="true">3.20.</strong> Drop</a></li><li class="chapter-item expanded "><a href="if-let.html"><strong aria-hidden="true">3.21.</strong> if let</a></li><li class="chapter-item expanded "><a href="trait-objects.html"><strong aria-hidden="true">3.22.</strong> Trait Objects</a></li><li class="chapter-item expanded "><a href="closures.html"><strong aria-hidden="true">3.23.</strong> Closures</a></li><li class="chapter-item expanded "><a href="ufcs.html"><strong aria-hidden="true">3.24.</strong> Universal Function Call Syntax</a></li><li class="chapter-item expanded "><a href="crates-and-modules.html"><strong aria-hidden="true">3.25.</strong> Crates and Modules</a></li><li class="chapter-item expanded "><a href="const-and-static.html"><strong aria-hidden="true">3.26.</strong> const and static</a></li><li class="chapter-item expanded "><a href="attributes.html"><strong aria-hidden="true">3.27.</strong> Attributes</a></li><li class="chapter-item expanded "><a href="type-aliases.html"><strong aria-hidden="true">3.28.</strong> type aliases</a></li><li class="chapter-item expanded "><a href="casting-between-types.html"><strong aria-hidden="true">3.29.</strong> Casting between types</a></li><li class="chapter-item expanded "><a href="associated-types.html"><strong aria-hidden="true">3.30.</strong> Associated Types</a></li><li class="chapter-item expanded "><a href="unsized-types.html"><strong aria-hidden="true">3.31.</strong> Unsized Types</a></li><li class="chapter-item expanded "><a href="operators-and-overloading.html"><strong aria-hidden="true">3.32.</strong> Operators and Overloading</a></li><li class="chapter-item expanded "><a href="deref-coercions.html"><strong aria-hidden="true">3.33.</strong> Deref coercions</a></li><li class="chapter-item expanded "><a href="macros.html"><strong aria-hidden="true">3.34.</strong> Macros</a></li><li class="chapter-item expanded "><a href="raw-pointers.html"><strong aria-hidden="true">3.35.</strong> Raw Pointers</a></li><li class="chapter-item expanded "><a href="unsafe.html"><strong aria-hidden="true">3.36.</strong> unsafe</a></li></ol></li><li class="chapter-item expanded "><a href="effective-rust.html"><strong aria-hidden="true">4.</strong> Effective Rust</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="the-stack-and-the-heap.html"><strong aria-hidden="true">4.1.</strong> The Stack and the Heap</a></li><li class="chapter-item expanded "><a href="testing.html"><strong aria-hidden="true">4.2.</strong> Testing</a></li><li class="chapter-item expanded "><a href="conditional-compilation.html"><strong aria-hidden="true">4.3.</strong> Conditional Compilation</a></li><li class="chapter-item expanded "><a href="documentation.html"><strong aria-hidden="true">4.4.</strong> Documentation</a></li><li class="chapter-item expanded "><a href="iterators.html"><strong aria-hidden="true">4.5.</strong> Iterators</a></li><li class="chapter-item expanded "><a href="concurrency.html"><strong aria-hidden="true">4.6.</strong> Concurrency</a></li><li class="chapter-item expanded "><a href="error-handling.html"><strong aria-hidden="true">4.7.</strong> Error Handling</a></li><li class="chapter-item expanded "><a href="choosing-your-guarantees.html"><strong aria-hidden="true">4.8.</strong> Choosing your Guarantees</a></li><li class="chapter-item expanded "><a href="ffi.html"><strong aria-hidden="true">4.9.</strong> FFI</a></li><li class="chapter-item expanded "><a href="borrow-and-asref.html"><strong aria-hidden="true">4.10.</strong> Borrow and AsRef</a></li><li class="chapter-item expanded "><a href="release-channels.html"><strong aria-hidden="true">4.11.</strong> Release Channels</a></li><li class="chapter-item expanded "><a href="using-rust-without-the-standard-library.html"><strong aria-hidden="true">4.12.</strong> Using Rust without the standard library</a></li><li class="chapter-item expanded "><a href="procedural-macros.html"><strong aria-hidden="true">4.13.</strong> Procedural Macros (and custom derive)</a></li></ol></li><li class="chapter-item expanded "><a href="glossary.html"><strong aria-hidden="true">5.</strong> Glossary</a></li><li class="chapter-item expanded "><a href="syntax-index.html"><strong aria-hidden="true">6.</strong> Syntax Index</a></li><li class="chapter-item expanded "><a href="bibliography.html"><strong aria-hidden="true">7.</strong> Bibliography</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
