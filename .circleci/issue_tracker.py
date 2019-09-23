#!/usr/bin/env python3
import glob
import re
import os
from datetime import date
import json

# pip3 install PyGithub
from github import Github

# Connect to the GitHub repo
gh = Github(os.environ['GITHUB_SECRET'])
repo = gh.get_repo(re
    .compile('git@github\.com:(\w+/\w+)\.git')
    .match(os.environ['CIRCLE_REPOSITORY_URL'])
    .groups(1)[0]
)
print('Connected to', repo)

# GitHub labels for issues
labels = {
    'TODO': ['tracker', 'refactor'],
    'OPT': ['tracker', 'performance'],
    'HACK': ['tracker', 'hack']
}

# GitHub users for emails
# HACK: Don't hard code the users
users = {
    '<remco@0x.org>': 'recmo',
    '<paul@0x.org>': 'pvienhage',
    '<mason@0x.org>': 'z2trillion',
}

# Translate labels to PyGitHub `Label`s
gh_labels = repo.get_labels()
gh_labels = {l.name: l for l in gh_labels}
labels = {k: [gh_labels[v] for v in v] for k, v in labels.items()}

# Translate users ot PyGitHub `User`s
users = {k: gh.get_user(v) for k, v in users.items()}

# Collect existing tracker issues
open_issues = []
for issue in repo.get_issues():
    if gh_labels['tracker'] in issue.labels:
        open_issues += [issue]
print(open_issues)

# Number of lines to give before and after the TODO comment.
CONTEXT_LINES = 5

# Rust like todos. For *.{rs}
rust_todo = re.compile('//\W*(TODO|HACK|OPT)\W*(.*)$')
rust_continuation = re.compile('//\W*(?!(TODO|HACK|OPT))(.*)$')

# Bash like todos. For *.{sh, yml, toml, py, Dockerfile, editorconfig, gitignore}
# TODO: `# TODO: {message}`

# Markdown like todos. For *.{md}
# TODO: `<!-- TODO: {message} -->`

# Markdown todo lists.
# TODO: `* [] {message}`

# Git current commit id
commit_hash = None
with os.popen('git rev-parse HEAD') as process:
    commit_hash = process.read().strip()
print('Commit hash:', commit_hash)

def git_blame(filename, line):
    # Git line numbers start at one
    command = (
        'git blame {filename} --line-porcelain -L {line},{line}'
        .format(
            filename=filename,
            line=line + 1
        )
    )
    with os.popen(command) as process:
        lines = process.readlines()
        result = dict()
        result['commit-hash'] = lines[0].split(' ')[0]
        for line in lines[1:-1]:
            line = line.replace('\n', '')
            [key, value] = line.split(' ', 1)
            result[key] = value
        result['author-time'] = int(result['author-time'])
        result['committer-time'] = int(result['committer-time'])
        return result

def get_context(filename, start, end):
    with open(filename, 'r') as file:
        lines = file.readlines()
        start = max(0, start)
        end = min(end, len(lines))
        return ''.join(lines[start:end])

def submit_issue(issue):
    issue['json'] = json.dumps(issue)
    issue['head'] = issue['issue'].split('\n')[0]
    issue['github-handle'] = users[issue['author-mail']].login
    issue['author-time-pretty'] = date.fromtimestamp(issue['author-time']).isoformat()
    issue['commit-hash-short'] = issue['commit-hash'][0:7]
    issue['line-one'] = issue['line'] + 1
    formatted = dict(
        title='{head}'.format(**issue),
        body='''
*On {author-time-pretty} @{github-handle} wrote in [`{commit-hash-short}`](https://github.com/{repo}/commit/{commit-hash}) “{summary}”:*

{issue}

```rust
{context}
```
*From [`{filename}:{line-one}`](https://github.com/{repo}/blob/{branch-hash}/{filename}#L{line-one})*

<!--{json}-->
'''.strip().format(**issue),
        assignee=users[issue['author-mail']],
        labels=labels[issue['kind']]
    )
    # TODO: Check if issue already exists
    print('Creating issue...')
    print(formatted['body'])
    #gh_issue = repo.create_issue(**formatted)
    #print(gh_issue)

def issues_from_file(filename):
    with open(filename, 'r') as file:
        line_number = 0
        issue = None
        kind = None
        issue_line = 0
        for line in file:
            match = rust_todo.search(line)
            continuation = rust_continuation.search(line)
            if match:
                issue = match.group(2)
                kind = match.group(1)
                issue_line = line_number
            elif issue and continuation:
                issue += '\n' + continuation.group(2)
            elif issue:
                result = git_blame(filename, issue_line)
                context = get_context(
                    filename,
                    issue_line - CONTEXT_LINES,
                    line_number + CONTEXT_LINES
                )
                result['filename'] = filename
                result['line'] = issue_line
                result['line_end'] = line_number
                result['kind'] = kind
                result['issue'] = issue
                result['context'] = context
                result['repo'] = repo.full_name
                result['branch-hash'] = commit_hash
                yield result
                issue = None
                kind = None
                issue_line = 0
            line_number += 1

def issues_from_glob(pattern):
    for filename in glob.iglob(pattern, recursive=True):
        for issue in issues_from_file(filename):
            yield issue

# Rust source code
for issue in issues_from_glob('algebra/u256/**/gcd*.rs'):
    submit_issue(issue)
    pass
