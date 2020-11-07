# Top Navigation
nav-top-authors = authors
nav-top-origins = origins
nav-top-warnings = warnings
nav-top-pairings = pairings
nav-top-characters = characters
nav-top-tags = tags
nav-top-search = search
nav-top-dashboard = dashboard

# Bottom Navigation
nav-bottom-rendered-in = rendered in: { $time }ms

# Pagination Navigation
nav-pagination-prev = prev
nav-pagination-next = next

# Story Information
story-info-chapters = { $chapters ->
        [zero]      no chapters
        [one]       1 chapter
        *[other]    { $chapters } chapters
    }
story-info-words = { $words ->
        [zero]      no words
        [one]       1 word
        *[other]    {$words} words
    }
story-tooltip-rating = rating: { $rating }
story-tooltip-state = state: { $state }
story-tooltip-warnings = warnings: { $warnings ->
        [zero]      none
        *[other]    using
    }
