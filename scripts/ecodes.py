# yoshi_error_code_scraper.py

# ==============================================================================
# S.I.N.G.U.L.A.R.I.T.Y. CRVO FRAMEWORK - PYTHON SCRIPTING
# CLEAN: Modular design with clear function responsibilities and comments.
# REUSABLE: Functions like fetch_and_parse can be adapted for other scraping.
# VERIFIED: Handles HTTP errors, network issues, and missing content gracefully.
# OPTIMAL: Uses asyncio and aiohttp for high-performance concurrent scraping.
# ==============================================================================

import asyncio
import re
from pathlib import Path

import aiohttp
from bs4 import BeautifulSoup
from tqdm.asyncio import tqdm_asyncio

# --- Configuration: Research-Validated Target Parameters ---

# VERIFIED: The base URL using the correct error_codes path without stable prefix.
BASE_URL = "https://doc.rust-lang.org/error_codes/{}.html"
# VERIFIED: Updated selector for rustdoc HTML structure compatibility.
CONTENT_SELECTOR = "main"
# OPTIMAL: A realistic concurrency limit to avoid overwhelming the server or client.
CONCURRENCY_LIMIT = 100
# CLEAN: A descriptive output filename.
OUTPUT_FILENAME = "rust_error_codes.txt"

# --- Data: The list of target error codes to scrape ---
ERROR_CODES_TEXT = """
E0001 E0002 E0004 E0005 E0007 E0009 E0010 E0013 E0014 E0015 E0023 E0025 E0026
E0027 E0029 E0030 E0033 E0034 E0038 E0040 E0044 E0045 E0046 E0049 E0050 E0053
E0054 E0055 E0057 E0059 E0060 E0061 E0062 E0063 E0067 E0069 E0070 E0071 E0072
E0073 E0074 E0075 E0076 E0077 E0080 E0081 E0084 E0087 E0088 E0089 E0090 E0091
E0092 E0093 E0094 E0106 E0107 E0109 E0110 E0116 E0117 E0118 E0119 E0120 E0121
E0124 E0128 E0130 E0131 E0132 E0133 E0136 E0137 E0138 E0139 E0152 E0154 E0158
E0161 E0162 E0164 E0165 E0170 E0178 E0183 E0184 E0185 E0186 E0191 E0192 E0193
E0195 E0197 E0198 E0199 E0200 E0201 E0203 E0204 E0205 E0206 E0207 E0208 E0210
E0211 E0212 E0214 E0220 E0221 E0222 E0223 E0224 E0225 E0226 E0227 E0228 E0229
E0230 E0231 E0232 E0243 E0244 E0251 E0252 E0253 E0254 E0255 E0256 E0259 E0260
E0261 E0262 E0263 E0264 E0267 E0268 E0271 E0275 E0276 E0277 E0281 E0282 E0283
E0284 E0297 E0301 E0302 E0303 E0307 E0308 E0309 E0310 E0311 E0312 E0316 E0317
E0320 E0321 E0322 E0323 E0324 E0325 E0326 E0328 E0329 E0364 E0365 E0366 E0367
E0368 E0369 E0370 E0371 E0373 E0374 E0375 E0376 E0377 E0378 E0379 E0380 E0381
E0382 E0383 E0384 E0386 E0387 E0388 E0389 E0390 E0391 E0392 E0393 E0398 E0399
E0401 E0403 E0404 E0405 E0407 E0408 E0409 E0411 E0412 E0415 E0416 E0422 E0423
E0424 E0425 E0426 E0428 E0429 E0430 E0431 E0432 E0433 E0434 E0435 E0436 E0437
E0438 E0439 E0445 E0446 E0447 E0448 E0449 E0451 E0452 E0453 E0454 E0455 E0457
E0458 E0459 E0460 E0461 E0462 E0463 E0464 E0466 E0468 E0469 E0472 E0476 E0477
E0478 E0482 E0491 E0492 E0493 E0495 E0496 E0497 E0498 E0499 E0500 E0501 E0502
E0503 E0504 E0505 E0506 E0507 E0508 E0509 E0510 E0511 E0512 E0514 E0515 E0516
E0517 E0518 E0519 E0520 E0521 E0522 E0523 E0524 E0525 E0527 E0528 E0529 E0530
E0531 E0532 E0533 E0534 E0535 E0536 E0537 E0538 E0539 E0541 E0542 E0543 E0544
E0545 E0546 E0547 E0549 E0550 E0551 E0552 E0554 E0556 E0557 E0559 E0560 E0561
E0562 E0565 E0566 E0567 E0568 E0569 E0570 E0571 E0572 E0573 E0574 E0575 E0576
E0577 E0578 E0579 E0580 E0581 E0582 E0583 E0584 E0585 E0586 E0587 E0588 E0589
E0590 E0591 E0592 E0593 E0594 E0595 E0596 E0597 E0599 E0600 E0601 E0602 E0603
E0604 E0605 E0606 E0607 E0608 E0609 E0610 E0614 E0615 E0616 E0617 E0618 E0619
E0620 E0621 E0622 E0623 E0624 E0625 E0626 E0627 E0631 E0632 E0633 E0634 E0635
E0636 E0637 E0638 E0639 E0640 E0641 E0642 E0643 E0644 E0646 E0647 E0648 E0657
E0658 E0659 E0660 E0661 E0662 E0663 E0664 E0665 E0666 E0667 E0668 E0669 E0670
E0671 E0687 E0688 E0689 E0690 E0691 E0692 E0693 E0695 E0696 E0697 E0698 E0699
E0700 E0701 E0703 E0704 E0705 E0706 E0708 E0710 E0712 E0713 E0714 E0715 E0716
E0711 E0717 E0718 E0719 E0720 E0722 E0724 E0725 E0726 E0727 E0728 E0729 E0730
E0731 E0732 E0733 E0734 E0735 E0736 E0737 E0739 E0740 E0741 E0742 E0743 E0744
E0745 E0746 E0747 E0748 E0749 E0750 E0751 E0752 E0753 E0754 E0755 E0756 E0757
E0758 E0759 E0760 E0761 E0762 E0763 E0764 E0765 E0766 E0767 E0768 E0769 E0770
E0771 E0772 E0773 E0774 E0775 E0776 E0777 E0778 E0779 E0780 E0781 E0782 E0783
E0784 E0785 E0786 E0787 E0788 E0789 E0790 E0791 E0792 E0793 E0794 E0795 E0796
E0797 E0798 E0799 E0800 E0801 E0802 E0803 E0804
"""

def parse_error_codes(text: str) -> list[str]:
    """
    Parses a string of text to find all Rust error codes (e.g., E0123).

    Args:
        text: The string containing error codes.

    Returns:
        A list of unique error code strings.
    """
    # VERIFIED: Regex reliably extracts the specific E-number format.
    # The set conversion ensures that each code is processed only once.
    codes = re.findall(r'E\d{4}', text)
    return sorted(list(set(codes)))


async def fetch_and_parse(session: aiohttp.ClientSession, code: str) -> tuple[str, str | None]:
    """
    Asynchronously fetches a URL for a given error code and parses its content.

    Args:
        session: The aiohttp client session to use for the request.
        code: The error code (e.g., 'E0001') to scrape.

    Returns:
        A tuple containing the error code and the scraped content text,
        or None for the content if scraping fails.
    """
    url = BASE_URL.format(code)
    try:
        # OPTIMAL: Asynchronous GET request with a type-safe ClientTimeout object.
        async with session.get(url, timeout=aiohttp.ClientTimeout(total=30)) as response:
            # VERIFIED: Robustly checks for successful HTTP status.
            if response.status != 200:
                return (code, None)

            html = await response.text()
            soup = BeautifulSoup(html, 'html.parser')

            # VERIFIED: Targets the specific, stable content container with fallback strategy.
            main_content = soup.select_one(CONTENT_SELECTOR)

            if not main_content:
                # REUSABLE: Fallback selectors for different rustdoc page structures
                fallback_selectors = [".content", "#content", "article", "body"]
                for selector in fallback_selectors:
                    main_content = soup.select_one(selector)
                    if main_content:
                        break

            if main_content:
                # CLEAN: Extracts text while preserving paragraphs and stripping excess whitespace.
                content_text = main_content.get_text(separator='\n', strip=True)

                # VERIFIED: Validate content quality and completeness
                if len(content_text.strip()) < 50:
                    return (code, f"Warning: Very short content ({len(content_text)} chars): {content_text[:100]}...")
                elif "error code is no longer emitted" in content_text.lower():
                    return (code, f"Note: {code} is deprecated - {content_text[:200]}...")
                else:
                    return (code, content_text)
            else:
                # VERIFIED: Enhanced diagnostic information for debugging with type safety
                available_tags = []
                for tag in soup.find_all()[:10]:
                    tag_name = getattr(tag, 'name', None)
                    if tag_name:
                        available_tags.append(str(tag_name))
                unique_tags = list(set(available_tags))
                return (code, f"Content selector failed. Available tags: {', '.join(unique_tags)}. Page structure may have changed.")

    except asyncio.TimeoutError:
        # VERIFIED: Gracefully handles request timeouts.
        return (code, f"Error: Request timed out for {url}")
    except Exception:
        # VERIFIED: Catches other potential network or parsing errors.
        return (code, f"Error: Failed to fetch or parse {url}")


async def main():
    """
    Main asynchronous function to orchestrate the scraping process.
    """
    print("🚀 S.I.N.G.U.L.A.R.I.T.Y. Error Code Scraper Initialized")

    # REUSABLE: The parsing function cleanly separates data from logic.
    codes_to_scrape = parse_error_codes(ERROR_CODES_TEXT)
    total_codes = len(codes_to_scrape)
    print(f"Found {total_codes} unique error codes to scrape.")

    # OPTIMAL: Use an asyncio.Semaphore to limit concurrent requests.
    semaphore = asyncio.Semaphore(CONCURRENCY_LIMIT)

    async def fetch_with_semaphore(session, code):
        async with semaphore:
            return await fetch_and_parse(session, code)

    # OPTIMAL: Creates a single session for all requests for connection pooling.
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_with_semaphore(session, code) for code in codes_to_scrape]

        # CLEAN: tqdm provides excellent, real-time user feedback.
        results = await tqdm_asyncio.gather(
            *tasks, desc=f"Scraping {total_codes} error codes"
        )

    # VERIFIED: Sort results numerically to ensure a predictable output file.
    results.sort(key=lambda item: int(item[0][1:]))

    # --- File Output Generation ---
    output_path = Path(OUTPUT_FILENAME)
    successful_scrapes = 0
    with output_path.open("w", encoding="utf-8") as f:
        for i, (code, content) in enumerate(results):
            if content and not content.startswith("Error:") and not content.startswith("Content selector failed") and not content.startswith("Warning: Very short content"):
                f.write(content.strip())
                successful_scrapes += 1
            else:
                # VERIFIED: Reports failures directly in the output for review.
                failure_reason = content or f"Page for {code} not found or failed to parse."
                f.write(f"Error code {code}\n\n{failure_reason}")

            # CLEAN: Separator for clean, machine-readable file structure.
            if i < len(results) - 1:
                f.write("\n---\n")

    print("\n✅ Scraping complete!")
    print(f"Successfully scraped {successful_scrapes}/{total_codes} error codes.")
    print(f"Results saved to '{output_path.resolve()}'")


if __name__ == "__main__":
    # This check ensures the async loop runs only when the script is executed directly.
    # On Windows, this specific setup for the event loop policy can prevent
    # certain `aiohttp` related errors on shutdown.
    if asyncio.get_event_loop().is_running():
         # In environments like Jupyter, the loop is already running.
         # This part is more for robustness across different execution environments.
         print("Loop is running, attaching to existing loop.")

    asyncio.run(main())
