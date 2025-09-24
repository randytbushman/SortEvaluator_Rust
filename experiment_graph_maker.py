import matplotlib.pyplot as plt
import pandas as pd
from typing import List, Optional

FIGURE_DPI = 600

# Monochrome encoding: all black, identity via dash patterns
pattern_map = {
    "QR Sort": (None, None),          # solid
    "Radix Sort": (0, (1.2, 1.2)),    # dotted
    "Counting Sort": (0, (3, 2, 1.2, 2)),  # dash-dot
    "Quicksort": (0, (6, 2)),         # dashed
    "Merge Sort": (0, (5, 2, 1, 2))   # long dash, short dash
}


def plot_from_csv(ax, csv_file: str, x_col: str,
                  include_cols: Optional[List[str]] = None,
                  x_scale: float = 1.0,
                  add_labels: bool = False) -> float:
    """
    Plot specified columns from a CSV file onto a given Axes subplot.

    :param ax: matplotlib Axes for plotting
    :param csv_file: Path to CSV file
    :param x_col: Column name for x-axis data
    :param include_cols: List of column names to include in plotting (in order). If None, include all except x_col.
    :param x_scale: Scaling factor applied to x-axis values
    :param add_labels: Whether to add direct end-labels on lines
    :return: Maximum y-value among plotted columns
    """
    df = pd.read_csv(csv_file)

    if include_cols is None:
        include_cols = [col for col in df.columns if col != x_col]

    x_values = df[x_col] / x_scale
    max_y = 0

    for col in include_cols:
        y_values = df[col] / 1000  # Convert to milliseconds

        ax.plot(
            x_values,
            y_values,
            label=col,
            color="black",
            linestyle="-" if pattern_map[col] == (None, None) else (0, pattern_map[col][1]),
            linewidth=2.0,
        )

        if add_labels:
            ax.annotate(
                col,
                xy=(x_values.iloc[-1], y_values.iloc[-1]),
                xytext=(6, 0),
                textcoords="offset points",
                va="center",
                fontsize=9,
                bbox=dict(boxstyle="round,pad=0.15", fc="white", ec="0.6", lw=0.5)
            )

        max_y = max(max_y, y_values.max())

    return max_y


def add_text_to_ax(ax, text, pos_x=0.95, pos_y=0.05):
    """
    Add annotation text to an Axes subplot in the bottom right.
    """
    ax.text(pos_x, pos_y, text, ha='right', va='bottom', transform=ax.transAxes,
            fontsize=12, color='black',
            bbox=dict(facecolor='white', alpha=0.5, edgecolor='none', boxstyle='round,pad=0.1'))


def make_qr_rs_graph():
    """
    Generate QR vs Radix plots with legends.
    """
    file_paths = [
        ("./results_qr_rs/0min_value-100000max_value.csv", "range = $10^5$"),
        ("./results_qr_rs/0min_value-1000000max_value.csv", "range = $10^6$"),
        ("./results_qr_rs/0min_value-10000000max_value.csv", "range = $10^7$"),
        ("./results_qr_rs/0min_value-100000000max_value.csv", "range = $10^8$"),
        ("./results_qr_rs/0min_value-1000000000max_value.csv", "range = $10^9$"),
        ("./results_qr_rs/0min_value-10000000000max_value.csv", "range = $10^{10}$"),
    ]

    labels = ["A", "B", "C", "D", "E", "F"]
    x_col = "Length"
    columns_to_plot = ["QR Sort", "Radix Sort"]

    fig, axes = plt.subplots(3, 2, figsize=(11, 8))
    max_y_values = []

    for ax, (csv_file, m_text), label in zip(axes.flatten(), file_paths, labels):
        max_y = plot_from_csv(ax, csv_file, x_col, columns_to_plot, x_scale=1e3, add_labels=False)
        max_y_values.append(max_y)
        ax.set_xlabel("Array Length ($10^3$)")
        ax.set_ylabel("Milliseconds")
        ax.set_title(f"{label}")
        add_text_to_ax(ax, m_text)

    global_max_y = max(max_y_values)
    for ax in axes.flatten():
        ax.set_xlim(0, 1_010)
        ax.set_ylim(0, global_max_y * 1.05)

    axes[0, 0].legend(loc='upper left', fontsize=12)

    fig.tight_layout()
    fig.savefig("figure_qr_vs_radix_6.png", dpi=FIGURE_DPI)
    fig.show()


def make_qr_qs_ms_graph():
    """
    Generate QR vs comparison-based plots (with optional direct labels).
    """
    filepath = "results_qr_ms_qs/0min_value-10000000000max_value.csv"

    x_col = "Length"
    columns_to_plot = ["QR Sort", "Quicksort", "Merge Sort", "Radix Sort"]

    fig, ax = plt.subplots(figsize=(10, 6))

    max_y = plot_from_csv(ax, filepath, x_col, columns_to_plot, x_scale=1e3, add_labels=True)
    ax.set_xlabel("Array Length ($10^3$)")
    ax.set_ylabel("Milliseconds")
    ax.set_title("QR Sort vs Comparison-Based Algorithms")
    add_text_to_ax(ax, "range = $10^{10}$")

    ax.set_xlim(0, 1_010)
    ax.set_ylim(0, max_y * 1.05)

    ax.legend(loc='upper left', fontsize=12)

    fig.tight_layout()
    fig.savefig("comparison_based_algs.png", dpi=FIGURE_DPI)
    fig.show()


def main() -> None:
    make_qr_rs_graph()
    make_qr_qs_ms_graph()


if __name__ == '__main__':
    main()
