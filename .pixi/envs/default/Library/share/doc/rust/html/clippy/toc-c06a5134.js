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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="usage.html"><strong aria-hidden="true">2.</strong> Usage</a></li><li class="chapter-item expanded "><a href="configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="lint_configuration.html"><strong aria-hidden="true">3.1.</strong> Lint Configuration</a></li></ol></li><li class="chapter-item expanded "><a href="lints.html"><strong aria-hidden="true">4.</strong> Clippy&#39;s Lints</a></li><li class="chapter-item expanded "><a href="attribs.html"><strong aria-hidden="true">5.</strong> Attributes for Crate Authors</a></li><li class="chapter-item expanded "><a href="continuous_integration/index.html"><strong aria-hidden="true">6.</strong> Continuous Integration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="continuous_integration/github_actions.html"><strong aria-hidden="true">6.1.</strong> GitHub Actions</a></li><li class="chapter-item expanded "><a href="continuous_integration/gitlab.html"><strong aria-hidden="true">6.2.</strong> GitLab CI</a></li><li class="chapter-item expanded "><a href="continuous_integration/travis.html"><strong aria-hidden="true">6.3.</strong> Travis CI</a></li></ol></li><li class="chapter-item expanded "><a href="development/index.html"><strong aria-hidden="true">7.</strong> Development</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/basics.html"><strong aria-hidden="true">7.1.</strong> Basics</a></li><li class="chapter-item expanded "><a href="development/adding_lints.html"><strong aria-hidden="true">7.2.</strong> Adding Lints</a></li><li class="chapter-item expanded "><a href="development/defining_lints.html"><strong aria-hidden="true">7.3.</strong> Defining Lints</a></li><li class="chapter-item expanded "><a href="development/writing_tests.html"><strong aria-hidden="true">7.4.</strong> Writing tests</a></li><li class="chapter-item expanded "><a href="development/lint_passes.html"><strong aria-hidden="true">7.5.</strong> Lint Passes</a></li><li class="chapter-item expanded "><a href="development/emitting_lints.html"><strong aria-hidden="true">7.6.</strong> Emitting lints</a></li><li class="chapter-item expanded "><a href="development/type_checking.html"><strong aria-hidden="true">7.7.</strong> Type Checking</a></li><li class="chapter-item expanded "><a href="development/trait_checking.html"><strong aria-hidden="true">7.8.</strong> Trait Checking</a></li><li class="chapter-item expanded "><a href="development/method_checking.html"><strong aria-hidden="true">7.9.</strong> Method Checking</a></li><li class="chapter-item expanded "><a href="development/macro_expansions.html"><strong aria-hidden="true">7.10.</strong> Macro Expansions</a></li><li class="chapter-item expanded "><a href="development/common_tools_writing_lints.html"><strong aria-hidden="true">7.11.</strong> Common Tools</a></li><li class="chapter-item expanded "><a href="development/infrastructure/index.html"><strong aria-hidden="true">7.12.</strong> Infrastructure</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/infrastructure/sync.html"><strong aria-hidden="true">7.12.1.</strong> Syncing changes between Clippy and rust-lang/rust</a></li><li class="chapter-item expanded "><a href="development/infrastructure/backport.html"><strong aria-hidden="true">7.12.2.</strong> Backporting Changes</a></li><li class="chapter-item expanded "><a href="development/infrastructure/changelog_update.html"><strong aria-hidden="true">7.12.3.</strong> Updating the Changelog</a></li><li class="chapter-item expanded "><a href="development/infrastructure/release.html"><strong aria-hidden="true">7.12.4.</strong> Release a New Version</a></li><li class="chapter-item expanded "><a href="development/infrastructure/book.html"><strong aria-hidden="true">7.12.5.</strong> The Clippy Book</a></li><li class="chapter-item expanded "><a href="development/infrastructure/benchmarking.html"><strong aria-hidden="true">7.12.6.</strong> Benchmarking Clippy</a></li></ol></li><li class="chapter-item expanded "><a href="development/proposals/index.html"><strong aria-hidden="true">7.13.</strong> Proposals</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/proposals/roadmap-2021.html"><strong aria-hidden="true">7.13.1.</strong> Roadmap 2021</a></li><li class="chapter-item expanded "><a href="development/proposals/syntax-tree-patterns.html"><strong aria-hidden="true">7.13.2.</strong> Syntax Tree Patterns</a></li></ol></li><li class="chapter-item expanded "><a href="development/the_team.html"><strong aria-hidden="true">7.14.</strong> The Team</a></li></ol></li></ol>';
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
