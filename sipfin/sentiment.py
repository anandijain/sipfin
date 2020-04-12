from transformers import pipeline
import pandas as pd


def add_sentiments(df: pd.DataFrame, col:str, label_col="sentiment_label", score_col="sentiment_score") -> pd.DataFrame:
    nlp = pipeline('sentiment-analysis')
    sentiments = [nlp(text)[0] for text in df[col]]

    labels = [s['label'] for s in sentiments]
    labels = pd.Series(labels, name=label_col)

    scores = [s['score'] for s in sentiments]
    scores = pd.Series(scores, name=score_col)

    return pd.concat([df, labels, scores], axis=1)

