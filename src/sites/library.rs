use leptos::prelude::*;

#[component]
pub fn Library() -> impl IntoView { 

    return view! {
        <header class="title-main--page--container library">
            <h1>LIBRARY</h1>
        </header>
        <section class="main--library-container">
            <div class="main--library-text">
                <p>
                    "This is my curated collection of both physical and digital resources a library that includes books(fiction and non-fiction), papers and materials I find interesting across various topics. Currently organized into: computer science, mathematics, political science, linguistics/languages, and more."
                </p>
            </div>

            <div class="collection-books">
                <section class="book-category">
                    <div class="category-content">

                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="featured-cs-book.webp"
                                    alt="Cover the C Programming Language - (K&R)"
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>
                            <ul class="book-list">
                                <h3>Computer Science & Programming</h3>
                                <li>"The Art of Computer Programming - Knuth"</li>
                                <li>"Clean Code - Martin"</li>
                                <li>
                                    "Design Patterns: Elements of Reusable Object-Oriented Software"
                                </li>
                                <li>"Introduction to Algorithms - CLRS"</li>
                                <li>
                                    "Code: The Hidden Language of Computer Hardware and Software - Petzold"
                                </li>
                                <li>"The C Programming Language - (K&R)"</li>
                                <li>"The Pragmatic Programmer - Andrew Hunt, David Thomas"</li>
                                <li>
                                    "Computer Systems: A Programmer’s Perspective - Randal Bryant, David O'Hallaron"
                                </li>
                                <li>
                                    "What Every Computer Scientist Should Know About Floating-Point Arithmetic"
                                </li>
                                <li>"The Mythical Man-Month - Fred Brooks"</li>
                                <li>"The Rust Programming Language"</li>
                                <li>"arXiv:1706.03762 Attention Is All You Need"</li>
                                <li>"Computer Networking: A Top-Down Approach - Kurose, Ross"</li>
                                <li>
                                    "Designing Data-Intensive Applications, 2nd Edition -  Martin Kleppmann, Chris Riccomini"
                                </li>
                                <li>
                                    "Quantum Computation and Quantum Information - Michael Nielsen, Isaac Chuang"
                                </li>
                            </ul>
                        </div>

                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="linearAlgebra.webp"
                                    alt="Cover of Linear Algebra Done Right - Axl"
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>

                            <ul class="book-list">
                                <h3>Mathematics, Logic & Formal Reasoning</h3>

                                <li>"Linear Algebra Done Right - Axl"</li>
                                <li>
                                    "Discrete Mathematical Structures - Bernard Kolman, Robert Busby, Sharon Ross"
                                </li>
                                <li>"Elementary Statistic - Neil Weiss"</li>
                                <li>"Hurley, Patrick J. – A Concise Introduction to Logic"</li>
                                <li>"Calculus Made Easy - Thompson"</li>
                                <li>"Graph Theory - Ronald Gould"</li>
                                <li>
                                    "How to Prove It: A Structured Approach - Daniel J. Velleman"
                                </li>
                                <li>"Barski, Conrad – Land of Lisp"</li>
                                <li>"Hart, William – College Algebra"</li>
                                <li>"Love, Clyde E. – Analytic Geometry"</li>
                                <li>"Daniel Kahneman - Thinking, Fast and Slow"</li>
                            </ul>
                        </div>
                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="leviathan.webp"
                                    alt="Cover of Leviathan - Thomas Hobbes"
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>
                            <ul class="book-list">
                                <h3>Philosophy, Politics & Social Theory</h3>

                                <li>"The Republic - Plato"</li>
                                <li>"Leviathan - Thomas Hobbes"</li>
                                <li>"The Social Contract - Jean-Jacques Rousseau"</li>
                                <li>"1984 - George Orwell"</li>
                                <li>"Animal Farm - George Orwell"</li>
                                <li>"The Age of Surveillance Capitalism"</li>
                                <li>"The End of History and the Last Man"</li>
                                <li>"Notes from Underground - Fyodor Dostoevsky"</li>
                                <li>
                                    "The Art of Strategy: A Game Theorist's Guide to Success in Business and Life"
                                </li>
                                <li>
                                    "The Black Swan: The Impact of the Highly Improbable - Nassim Taleb"
                                </li>
                            </ul>
                        </div>
                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="singularity.webp"
                                    alt="Cover the The Singularity is Nearer Kurzweil, Ray"
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>

                            <ul class="book-list">
                                <h3>Futurism, AI & Technology</h3>

                                <li>
                                    "A Brief History of Intelligence: Evolution, AI, and the Five Breakthroughs That Made Our Brains"
                                </li>
                                <li>
                                    "The Age of Em: Work, Love and Life when Robots Rule the Earth"
                                </li>
                                <li>
                                    "Asimov, Isaac – Before the Golden Age: A Science Fiction Anthology of the 1930's "
                                </li>
                                <li>"Kurzweil, Ray – The Singularity is Near "</li>
                                <li>"Kurzweil, Ray – The Singularity is Nearer "</li>
                                <li>
                                    "Nexus: A Brief History of Information Networks from the Stone Age to AI"
                                </li>
                                <li>"Sapiens: A Brief History of Humankind"</li>
                                <li>"The Last Question - Isaac Asimov"</li>
                            </ul>
                        </div>
                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="brothers-karamazov.webp"
                                    alt="Cover The Brothers Karamazov - Fyodor Dostoevsky"
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>
                            <ul class="book-list">
                                <h3>Russian Literature</h3>

                                <li>"Crime and Punishment - Fyodor Dostoevsky"</li>
                                <li>"The Idiot - Fyodor Dostoevsky"</li>
                                <li>"The Brothers Karamazov - Fyodor Dostoevsky"</li>
                                <li>"Oblomov - Ivan Goncharov"</li>
                                <li>"Life and Fate - Vasily Grossman"</li>
                                <li>"The Foundation Pit - Andrei Platonov"</li>
                                <li>"The Cherry Orchard - Anton Chekhov"</li>

                            </ul>
                        </div>
                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="great-tales.webp"
                                    alt="Cover Lovecraft, H. P. – Great Tales of Horror"
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>
                            <ul class="book-list">
                                <h3>World Literature & Classics</h3>

                                <li>"MEDITATIONS, Marcus Aurelius"</li>
                                <li>"Homer – The Iliad"</li>
                                <li>
                                    "Tolkien, J. R. R. – The Lord of the Rings, One-Volume Edition"
                                </li>
                                <li>"Tolkien, J. R. R. – The Silmarillion"</li>
                                <li>"Lovecraft, H. P. – Great Tales of Horror"</li>
                                <li>
                                    "Lyon, Pamela – French Short Stories; Nouvelles françaises"
                                </li>
                                <li>"Lucretius – On the Nature of Things"</li>
                                <li>
                                    "Euripides – Three Plays of Euripides: Alcestis, Medea, the Bacchae"
                                </li>
                                <li>
                                    "García Márquez, Gabriel – One Hundred Years of Solitude"
                                </li>
                                <li>"Alighieri, Dante – The Inferno"</li>
                                <li>"The Brothers Grimm Fairy Tales"</li>
                            </ul>
                        </div>
                        <div class="book-category--order">

                            <aside class="featured-book">
                                <img
                                    src="latin.webp"
                                    alt="Cover Wheelock, Frederic M. – Latin, an Introductory Course Based on Ancient Authors "
                                    class="book-cover"
                                    loading="lazy"
                                />
                            </aside>
                            <ul class="book-list">
                                <h3>Language & Linguistics</h3>

                                <li>
                                    "Dalby, Andrew – Dictionary of Languages, the Definitive Reference to More than 400 Languages "
                                </li>
                                <li>
                                    "Watkins, Calvert – The American Heritage Dictionary of Indo-European Roots "
                                </li>
                                <li>
                                    "Wheelock, Frederic M. – Latin, an Introductory Course Based on Ancient Authors "
                                </li>
                                <li>
                                    "Comfraterity of Christian Doctrine – The New Testament of Our Lord and Savior Jesus Christ Translated from the Latin Vulgate"
                                </li>
                                <li>
                                    "Whitney, William Dwight – The Roots, Verb-Forms, And Primary Derivatives of the Sankstrit Language: A Supplement to his Sanskrit Grammar"
                                </li>
                            </ul>
                        </div>

                    </div>

                </section>

            </div>
        </section>
    }
}
