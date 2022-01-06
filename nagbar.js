var nagbar = document.createElement('div');
nagbar.id = 'nagbar';
nagbar.className = 'nagbar';
nagbar.appendChild(document.createTextNode(' Donate to help me work on this book. Thanks!'));
nagbar.insertAdjacentHTML('afterbegin', '<a class="github-button" href="https://github.com/sponsors/inodentry" data-icon="octicon-heart" data-size="small" aria-label="Sponsor @inodentry on GitHub">GitHub Sponsors</a>');
document.getElementById('page-wrapper').appendChild(nagbar);
