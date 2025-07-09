import matplotlib.pyplot as plt
import pandas as pd
from typing import List, Optional


def plot_from_csv(ax, csv_file: str, x_col: str, exclude_cols: Optional[List[str]] = None, x_scale: float = 1.0) -> float:
    """
    Plot data from a CSV file onto a given Axes subplot, excluding specified columns.

    :param ax: matplotlib Axes for plotting
    :param csv_file: Path to CSV file
    :param x_col: Column name for x-axis data
    :param exclude_cols: List of column names to exclude from plotting
    :param x_scale: Scaling factor applied to x-axis values
    :return: Maximum y-value among plotted columns
    """
    df = pd.read_csv(csv_file)

    if exclude_cols is None:
        exclude_cols = []

    y_cols = [col for col in df.columns if col not in exclude_cols + [x_col]]
    x_values = df[x_col] / x_scale

    max_y = 0
    for col in y_cols:
        y_values = df[col] / 1000  # Convert to milliseconds
        ax.plot(x_values, y_values, label=col)
        max_y = max(max_y, y_values.max())

    return max_y


def add_text_to_ax(ax, text, pos_x=0.95, pos_y=0.05):
    """
    Add annotation text to an Axes subplot in the bottom right.

    :param ax: matplotlib Axes to annotate
    :param text: Text to add
    :param pos_x: X-axis position in axes coordinates (default = 0.95)
    :param pos_y: Y-axis position in axes coordinates (default = 0.05)
    """
    ax.text(pos_x, pos_y, text, ha='right', va='bottom', transform=ax.transAxes,
            fontsize=12, color='black',
            bbox=dict(facecolor='white', alpha=0.5, edgecolor='none', boxstyle='round,pad=0.1'))


def main():
    """
    Main function to generate and save the plots.
    """
    file_paths = [
        #("./results/0min_value-10000max_value.csv", "m = 10,000"),
        ("./results/0min_value-100000max_value.csv", "m = 100,000"),
        ("./results/0min_value-1000000max_value.csv", "m = 1,000,000"),
        ("./results/0min_value-10000000max_value.csv", "m = 10,000,000"),
        ("./results/0min_value-100000000max_value.csv", "m = 100,000,000"),
        ("./results/0min_value-1000000000max_value.csv", "m = 1,000,000,000"),
        ("./results/0min_value-10000000000max_value.csv", "m = 10,000,000,000"),
        ("./results/0min_value-100000000000max_value.csv", "m = 100,000,000,000"),
        ("./results/0min_value-1000000000000max_value.csv", "m = 1,000,000,000,000"),
        ("./results/0min_value-10000000000000max_value.csv", "m = 10,000,000,000,000"),
    ]

    labels = ["A", "B", "C", "D", "E", "F", "G", "H"]
    x_col = "Length"
    exclude_cols = ["Quicksort", "Merge Sort", "Counting Sort"]

    fig, axes = plt.subplots(4, 2, figsize=(14, 10))
    max_y_values = []

    for ax, (csv_file, m_text), label in zip(axes.flatten(), file_paths, labels):
        max_y = plot_from_csv(ax, csv_file, x_col, exclude_cols, x_scale=1e3)
        max_y_values.append(max_y)
        ax.set_xlabel("Array Length ($10^3$)")
        ax.set_ylabel("Milliseconds")
        ax.set_title(f"{label}")
        add_text_to_ax(ax, m_text)

    global_max_y = max(max_y_values)
    for ax in axes.flatten():
        ax.set_ylim(0, global_max_y * 1.05)

    axes[0, 0].legend(loc='upper left', fontsize=12)

    fig.tight_layout()
    fig.savefig("figure_qr_vs_radix_8.png", dpi=600)
    fig.show()


if __name__ == '__main__':
    main()
